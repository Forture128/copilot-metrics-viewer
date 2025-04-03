"""
GitHub Collector - A comprehensive data collection tool for GitHub data using PyGithub.

This module provides functionality to collect data from GitHub using both REST and GraphQL APIs,
with features like concurrency, rate limiting, fault tolerance, and caching.
"""

import os
import time
import asyncio
import logging
from typing import Dict, List, Any, Optional
from dataclasses import dataclass
from concurrent.futures import ThreadPoolExecutor
import json
from pathlib import Path
import random
from functools import wraps
from datetime import datetime, timezone
from dotenv import load_dotenv

# PyGithub imports
from github import Github, Auth, RateLimitExceededException
from github.Repository import Repository
from github.Organization import Organization
from github.PaginatedList import PaginatedList
from github.GithubObject import NotSet
from github.NamedUser import NamedUser
from github.Team import Team
from github.Workflow import Workflow
from github.WorkflowRun import WorkflowRun
from github.GitRelease import GitRelease
from github.Deployment import Deployment
from github.Branch import Branch
from github.Issue import Issue
from github.PullRequest import PullRequest
from github.Commit import Commit

# Local imports
from src.common.github_client import GithubClient
from src.common.github_graphql import GitHubGraphQLClient
from src.utils.loggers import get_logger


# Create logger
logger = get_logger("github_collector")

# Constants for rate limiting and retries
DEFAULT_RETRY_COUNT = 3
DEFAULT_RETRY_BACKOFF = 2.0
DEFAULT_CONCURRENT_REQUESTS = 5
DEFAULT_RATE_LIMIT_BUFFER = 100  # Keep 100 requests in reserve
DEFAULT_CACHE_DIR = Path("data/github_cache")
DEFAULT_CACHE_TTL = 3600  # 1 hour in seconds

load_dotenv()


@dataclass
class RateLimitStatus:
    """Track GitHub API rate limit status"""

    limit: int
    remaining: int
    reset_time: datetime
    used: int

    @classmethod
    def from_github_rate_limit(cls, rate_limit):
        """Create from PyGithub rate_limit object"""
        return cls(
            limit=rate_limit.core.limit,
            remaining=rate_limit.core.remaining,
            reset_time=rate_limit.core.reset,
            used=rate_limit.core.used,
        )

    def should_wait(self, buffer: int = DEFAULT_RATE_LIMIT_BUFFER) -> bool:
        """Check if we should wait for rate limit to reset"""
        return self.remaining <= buffer

    def wait_time(self) -> float:
        """Calculate how long to wait for rate limit reset"""
        now = datetime.now(timezone.utc)
        if self.reset_time > now:
            return (self.reset_time - now).total_seconds() + 5  # Add 5 seconds buffer
        return 0


def retry_on_exception(
    max_retries: int = DEFAULT_RETRY_COUNT,
    backoff_factor: float = DEFAULT_RETRY_BACKOFF,
    exceptions=(Exception,),
):
    """Decorator for retrying operations on exceptions with exponential backoff"""

    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            last_exception = None
            for attempt in range(max_retries + 1):
                try:
                    return func(*args, **kwargs)
                except exceptions as e:
                    last_exception = e
                    if attempt < max_retries:
                        sleep_time = backoff_factor**attempt + random.uniform(0, 1)
                        logger.warning(
                            f"Attempt {attempt + 1}/{max_retries + 1} failed: {str(e)}. "
                            f"Retrying in {sleep_time:.2f} seconds..."
                        )
                        time.sleep(sleep_time)
                    else:
                        logger.error(
                            f"Failed after {max_retries + 1} attempts: {str(e)}"
                        )
            raise last_exception

        return wrapper

    return decorator


def rate_limit_aware(buffer: int = DEFAULT_RATE_LIMIT_BUFFER):
    """Decorator to make functions aware of GitHub rate limits"""

    def decorator(func):
        @wraps(func)
        def wrapper(self, *args, **kwargs):
            # Check if we're using a GitHub client
            if hasattr(self, "_github_client") and self._github_client:
                # Get current rate limit
                rate_limit = RateLimitStatus.from_github_rate_limit(
                    self._github_client.client.get_rate_limit()
                )

                # If we're close to limit, wait until reset
                if rate_limit.should_wait(buffer):
                    wait_time = rate_limit.wait_time()
                    if wait_time > 0:
                        logger.warning(
                            f"Rate limit nearly reached ({rate_limit.remaining}/{rate_limit.limit}). "
                            f"Waiting {wait_time:.2f} seconds for reset."
                        )
                        time.sleep(wait_time)

            return func(self, *args, **kwargs)

        return wrapper

    return decorator


class CacheManager:
    """Manage caching for GitHub API responses"""

    def __init__(
        self, cache_dir: Path = DEFAULT_CACHE_DIR, ttl: int = DEFAULT_CACHE_TTL
    ):
        self.cache_dir = cache_dir
        self.ttl = ttl
        self.cache_dir.mkdir(parents=True, exist_ok=True)

    def get_cache_path(self, key: str) -> Path:
        """Get path for a cache key"""
        # Create a safe filename from the key
        safe_key = "".join(c if c.isalnum() else "_" for c in key)
        return self.cache_dir / f"{safe_key}.json"

    def get(self, key: str) -> Optional[Dict[str, Any]]:
        """Get cached data if it exists and is fresh"""
        cache_path = self.get_cache_path(key)

        if not cache_path.exists():
            return None

        try:
            stats = cache_path.stat()
            # Check if cache is expired
            if time.time() - stats.st_mtime > self.ttl:
                return None

            with open(cache_path, "r") as f:
                return json.load(f)
        except Exception as e:
            logger.warning("Error reading cache for %s: %s", key, str(e))
            return None

    def set(self, key: str, data: Any) -> None:
        """Store data in cache"""
        cache_path = self.get_cache_path(key)

        try:
            with open(cache_path, "w") as f:
                json.dump(data, f, cls=GitHubObjectEncoder)
        except Exception as e:
            logger.warning(f"Error writing cache for {key}: {str(e)}")


class GitHubObjectEncoder(json.JSONEncoder):
    """Custom JSON encoder for GitHub API objects"""

    def default(self, obj):
        if isinstance(
            obj,
            (
                Repository,
                Organization,
                NamedUser,
                Team,
                Workflow,
                WorkflowRun,
                GitRelease,
                Deployment,
                Branch,
                Issue,
                PullRequest,
                Commit,
            ),
        ):
            # Convert GitHub objects to dictionaries with their attributes
            return {k: v for k, v in obj.__dict__.items() if not k.startswith("_")}
        elif isinstance(obj, PaginatedList):
            # Convert PaginatedList to list
            return list(obj)
        elif isinstance(obj, datetime):
            # Convert datetime to ISO format string
            return obj.isoformat()
        return super().default(obj)


class GitHubCollector:
    """Main collector class for GitHub data"""

    def __init__(
        self,
        token: Optional[str] = None,
        concurrent_requests: int = DEFAULT_CONCURRENT_REQUESTS,
        use_cache: bool = True,
        cache_ttl: int = DEFAULT_CACHE_TTL,
        rate_limit_buffer: int = DEFAULT_RATE_LIMIT_BUFFER,
        org_name: str = "github",
    ):
        # Load token from env if not provided
        self.token = token or os.getenv("GITHUB_TOKEN")
        if not self.token:
            raise ValueError(
                "GitHub token is required. Set GITHUB_TOKEN environment variable or pass token."
            )

        # Initialize clients
        self._github_client = GithubClient(self.token)
        self._github_client.connect()

        self._graphql_client = GitHubGraphQLClient(token=self.token)

        # Set up executors for concurrent operations
        self.concurrent_requests = concurrent_requests
        self._executor = ThreadPoolExecutor(max_workers=concurrent_requests)

        # Set up caching
        self.use_cache = use_cache
        if use_cache:
            self.cache = CacheManager(ttl=cache_ttl)

        # Rate limiting settings
        self.rate_limit_buffer = rate_limit_buffer

        # Track collected repositories
        self._collected_repos = set()

        # Init Organization
        self.org = self.get_organization(org_name)

    def __del__(self):
        """Clean up resources"""
        if hasattr(self, "_executor"):
            self._executor.shutdown(wait=False)
        if hasattr(self, "_github_client") and self._github_client:
            self._github_client.client.close()

    @rate_limit_aware()
    @retry_on_exception(exceptions=(RateLimitExceededException, Exception))
    def get_organization(self, org_name: str) -> Organization:
        """Get GitHub organization"""
        return self._github_client.get_organization(org_name)

    @rate_limit_aware()
    @retry_on_exception(exceptions=(RateLimitExceededException, Exception))
    def get_repositories(
        self,
        org_name: str,
        type: str = "all",
        sort: str = "full_name",
        direction: str = "asc",
    ) -> List[Dict[str, Any]]:
        """Get all repositories for an organization with metadata"""
        # Use cache if enabled
        if self.use_cache:
            cache_key = f"repos_{org_name}_{type}_{sort}_{direction}"
            cached_data = self.cache.get(cache_key)
            if cached_data:
                logger.info(f"Using cached repository data for {org_name}")
                return cached_data

        # Get repositories
        repos = list(self.org.get_repos(type=type, sort=sort, direction=direction))

        # Testing
        repos = repos[:1]

        # Convert to serializable format with key metadata
        result = []
        for repo in repos:
            repo_data = {
                "id": repo.id,
                "name": repo.name,
                "full_name": repo.full_name,
                "description": repo.description,
                "html_url": repo.html_url,
                "created_at": repo.created_at.isoformat() if repo.created_at else None,
                "updated_at": repo.updated_at.isoformat() if repo.updated_at else None,
                "pushed_at": repo.pushed_at.isoformat() if repo.pushed_at else None,
                "language": repo.language,
                "stargazers_count": repo.stargazers_count,
                "forks_count": repo.forks_count,
                "size": repo.size,
                "default_branch": repo.default_branch,
                "archived": repo.archived,
                "private": repo.private,
            }
            result.append(repo_data)

        # Cache result if enabled
        if self.use_cache:
            self.cache.set(cache_key, result)

        return result

    @rate_limit_aware()
    @retry_on_exception(exceptions=(RateLimitExceededException, Exception))
    def get_repository_contributors(
        self, owner: str, repo: str
    ) -> List[Dict[str, Any]]:
        """Get contributors for a repository"""
        cache_key = f"contributors_{owner}_{repo}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        # Get repository and contributors
        repository = self._github_client.client.get_repo(f"{owner}/{repo}")
        contributors = list(repository.get_contributors())

        # Convert to serializable format
        result = []
        for contributor in contributors:
            contrib_data = {
                "id": contributor.id,
                "login": contributor.login,
                "contributions": contributor.contributions,
                "type": contributor.type,
                "site_admin": contributor.site_admin,
                "html_url": contributor.html_url,
            }
            result.append(contrib_data)

        if self.use_cache:
            self.cache.set(cache_key, result)

        return result

    async def collect_all_data(self, org_name: str) -> Dict[str, Any]:
        """
        Collect comprehensive data from GitHub for an organization

        This is an asynchronous method that collects:
        - Organization details
        - All repositories
        - Contributors for each repository
        - Pull requests for each repository
        - Issues for each repository
        - Commits for each repository

        Args:
            org_name: Name of the GitHub organization

        Returns:
            Dictionary with all collected data
        """
        # Collect organization data
        logger.info(f"Collecting data for organization: {org_name}")

        # Get repositories (this will use REST API with PyGithub)
        repositories = self.get_repositories(org_name)
        # repo_names = [repo["name"] for repo in repositories]
        logger.info(f"Found {len(repositories)} repositories")

        # Use asyncio and ThreadPoolExecutor to collect data concurrently
        results = {
            "organization": org_name,
            "repositories": repositories,
            "contributors": {},
            "pull_requests": {},
            "issues": {},
            "commits": {},
        }

        # Collect contributors for each repository concurrently
        loop = asyncio.get_event_loop()

        # Helper function to collect contributor data
        async def collect_contributor_data():
            tasks = []
            for repo in repositories:
                repo_name = repo["name"]
                tasks.append(
                    loop.run_in_executor(
                        self._executor,
                        self.get_repository_contributors,
                        org_name,
                        repo_name,
                    )
                )

            contributors_results = await asyncio.gather(*tasks, return_exceptions=True)

            for i, repo in enumerate(repositories):
                repo_name = repo["name"]
                if isinstance(contributors_results[i], Exception):
                    logger.error(
                        f"Error collecting contributors for {repo_name}: {str(contributors_results[i])}"
                    )
                    results["contributors"][repo_name] = []
                else:
                    results["contributors"][repo_name] = contributors_results[i]

        # Execute collection tasks
        await collect_contributor_data()

        logger.info(f"Data collection complete for {org_name}")
        return results

    def collect_all_data_sync(self, org_name: str) -> Dict[str, Any]:
        """Synchronous wrapper for collect_all_data"""
        loop = asyncio.new_event_loop()
        try:
            return loop.run_until_complete(self.collect_all_data(org_name))
        finally:
            loop.close()

    def save_data_to_json(self, data: Dict[str, Any], filepath: str) -> None:
        """Save collected data to a JSON file"""
        try:
            # Ensure directory exists
            Path(filepath).parent.mkdir(parents=True, exist_ok=True)

            # Save data using custom encoder
            with open(filepath, "w") as f:
                json.dump(data, f, indent=2, cls=GitHubObjectEncoder)

            logger.info(f"Data saved to {filepath}")
        except Exception as e:
            logger.error(f"Error saving data to {filepath}: {str(e)}")
            raise


def main():
    """Main function to demonstrate GitHub collector usage"""
    import argparse

    parser = argparse.ArgumentParser(description="GitHub Data Collector")
    parser.add_argument("--org", required=True, help="GitHub organization name")
    parser.add_argument(
        "--token", help="GitHub token (optional, will use env var if not provided)"
    )
    parser.add_argument(
        "--output", default="data/github_data.json", help="Output file path"
    )
    parser.add_argument(
        "--concurrent",
        type=int,
        default=DEFAULT_CONCURRENT_REQUESTS,
        help="Number of concurrent requests",
    )
    parser.add_argument("--no-cache", action="store_true", help="Disable caching")
    args = parser.parse_args()

    # Initialize collector
    collector = GitHubCollector(
        token=args.token,
        concurrent_requests=args.concurrent,
        use_cache=not args.no_cache,
        org_name=args.org,
    )

    # Collect data
    data = collector.collect_all_data_sync(args.org)

    # Save data
    collector.save_data_to_json(data, args.output)

    print(f"GitHub data collected and saved to {args.output}")


if __name__ == "__main__":
    main()
