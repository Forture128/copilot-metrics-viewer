"""
Main GitHub Collector implementation with specific data collection methods
"""

from typing import List, Dict, Any, Optional
from datetime import datetime, timedelta
import json
from pathlib import Path
from src.github_collector_v2.base_collector import BaseGitHubCollector
import logging
import os

logger = logging.getLogger(__name__)


class GitHubCollector(BaseGitHubCollector):
    """Main GitHub collector for DORA metrics and repository data"""

    async def get_repositories(
        self, repo_type: str = "all", limit: Optional[int] = None, batch_size: int = 100
    ) -> List[Dict[str, Any]]:
        """
        Get all repositories with pagination

        Args:
            repo_type: Type of repositories to fetch (all, public, private, etc.)
            limit: Maximum number of repositories to return
            batch_size: Number of repositories per page

        Returns:
            List of repository data dictionaries
        """
        # Try cache first
        cache_key = self._cache_key(["repos", repo_type, str(limit)])
        cached_data = self._get_from_cache(cache_key)
        if cached_data:
            return cached_data

        # Determine page size - use smaller page size if limit is specified
        page_size = min(100, limit) if limit else 100

        # Create query template based on repository type
        query_template = """
            query($org: String!, $cursor: String, $pageSize: Int!) {
                organization(login: $org) {
                    repositories(first: $pageSize, after: $cursor, orderBy: {field: CREATED_AT, direction: DESC}%s) {
                        pageInfo {
                            hasNextPage
                            endCursor
                        }
                        nodes {
                            id
                            name
                            url
                            description
                            createdAt
                            updatedAt
                            primaryLanguage {
                                name
                            }
                            defaultBranchRef {
                                name
                            }
                        }
                    }
                }
            }
        """

        # Add repository type filter if needed
        privacy_filter = ""
        if repo_type.lower() == "public":
            privacy_filter = ", privacy: PUBLIC"
        elif repo_type.lower() == "private":
            privacy_filter = ", privacy: PRIVATE"

        # Format the query with the appropriate filter
        query = query_template % privacy_filter

        repos = []
        cursor = None

        try:
            while True:
                variables = {
                    "org": self.org_name,
                    "cursor": cursor,
                    "pageSize": page_size,
                }
                result = await self.execute_graphql(query, variables, use_cache=False)

                # Check if the response contains the expected data
                if not result.get("organization", {}).get("repositories"):
                    logger.warning(
                        "Unexpected response format when fetching repositories"
                    )
                    break

                page_info = result["organization"]["repositories"]["pageInfo"]
                current_repos = result["organization"]["repositories"]["nodes"]
                repos.extend(current_repos)

                # Apply limit if specified
                if limit and len(repos) >= limit:
                    repos = repos[:limit]
                    break

                # Check if there are more pages
                if not page_info["hasNextPage"]:
                    break

                # If we're close to the limit, adjust the page size for the next request
                if limit:
                    remaining = limit - len(repos)
                    if remaining <= 0:
                        break
                    page_size = min(100, remaining)
                    variables["pageSize"] = page_size

                cursor = page_info["endCursor"]

                # Check rate limit after each page
                await self.check_rate_limit()

            # Cache the results
            if repos:
                self._set_cache(cache_key, repos)

            return repos

        except Exception as e:
            logger.error("Failed to get repositories: %s", str(e))
            raise

    async def get_repository_data(
        self, repo_name: str, days_lookback: int = 90, force_refresh: bool = False
    ) -> Dict[str, Any]:
        """
        Get comprehensive repository data including PRs, issues, and deployments

        Args:
            repo_name: Name of the repository
            days_lookback: Number of days to look back for data (default: 90)
            force_refresh: Whether to bypass cache and fetch fresh data

        Returns:
            Dictionary containing repository data
        """
        # Try cache first, unless force_refresh is True
        if not force_refresh:
            cache_key = self._cache_key(["repo_data", repo_name, str(days_lookback)])
            cached_data = self._get_from_cache(cache_key)
            if cached_data:
                return cached_data

        # Set up variables for the query
        variables = {"owner": self.org_name, "name": repo_name}

        # Add cutoff date to variables if needed
        use_date_filter = days_lookback > 0
        if use_date_filter:
            cutoff_date = (datetime.now() - timedelta(days=days_lookback)).strftime(
                "%Y-%m-%dT%H:%M:%SZ"
            )
            variables["since"] = cutoff_date

        # Define the history part of the query based on whether we're using date filtering
        history_part = (
            "history(first: 50, since: $since)"
            if use_date_filter
            else "history(first: 50)"
        )

        # Single query template with conditional part for history
        query = f"""
            query($owner: String!, $name: String!{", $since: GitTimestamp!" if use_date_filter else ""}) {{
                repository(owner: $owner, name: $name) {{
                    name
                    url
                    description
                    createdAt
                    updatedAt
                    pullRequests(first: 100, orderBy: {{field: CREATED_AT, direction: DESC}}, states: [OPEN, CLOSED, MERGED]) {{
                        totalCount
                        nodes {{
                            number
                            title
                            createdAt
                            mergedAt
                            closedAt
                            state
                            additions
                            deletions
                            changedFiles
                            mergeCommit {{
                                oid
                                committedDate
                            }}
                            commits(first: 1) {{
                                totalCount
                                nodes {{
                                    commit {{
                                        oid
                                        committedDate
                                        message
                                    }}
                                }}
                            }}
                            reviews(first: 10) {{
                                totalCount
                                nodes {{
                                    state
                                    submittedAt
                                    author {{
                                        login
                                    }}
                                }}
                            }}
                            timelineItems(first: 50, itemTypes: [MERGED_EVENT, CLOSED_EVENT, REOPENED_EVENT]) {{
                                nodes {{
                                    __typename
                                    ... on MergedEvent {{
                                        createdAt
                                    }}
                                    ... on ClosedEvent {{
                                        createdAt
                                    }}
                                    ... on ReopenedEvent {{
                                        createdAt
                                    }}
                                }}
                            }}
                        }}
                    }}
                    issues(first: 100, orderBy: {{field: CREATED_AT, direction: DESC}}, states: [OPEN, CLOSED]) {{
                        totalCount
                        nodes {{
                            number
                            title
                            createdAt
                            closedAt
                            state
                            labels(first: 10) {{
                                nodes {{
                                    name
                                    color
                                }}
                            }}
                            timelineItems(first: 30, itemTypes: [CLOSED_EVENT, REOPENED_EVENT]) {{
                                nodes {{
                                    __typename
                                    ... on ClosedEvent {{
                                        createdAt
                                        closer {{
                                            __typename
                                        }}
                                    }}
                                    ... on ReopenedEvent {{
                                        createdAt
                                    }}
                                }}
                            }}
                        }}
                    }}
                    deployments(first: 100, orderBy: {{field: CREATED_AT, direction: DESC}}) {{
                        totalCount
                        nodes {{
                            id
                            state
                            createdAt
                            environment
                            latestStatus {{
                                state
                                description
                                createdAt
                                logUrl
                                environmentUrl
                            }}
                            commit {{
                                oid
                                message
                            }}
                            ref {{
                                name
                            }}
                            task
                            payload
                        }}
                    }}
                    releases(first: 50, orderBy: {{field: CREATED_AT, direction: DESC}}) {{
                        totalCount
                        nodes {{
                            name
                            tagName
                            createdAt
                            publishedAt
                            isPrerelease
                            isDraft
                            description
                        }}
                    }}
                    vulnerabilityAlerts(first: 100) {{
                        nodes {{
                            securityVulnerability {{
                                severity
                                package {{
                                    name
                                }}
                            }}
                        }}
                    }}
                    defaultBranchRef {{
                        name
                        target {{
                            ... on Commit {{
                                {history_part} {{
                                    totalCount
                                    nodes {{
                                        committedDate
                                        message
                                        changedFiles
                                        additions
                                        deletions
                                    }}
                                }}
                                checkSuites(first: 50) {{
                                    nodes {{
                                        status
                                        conclusion
                                        workflowRun {{
                                            workflow {{
                                                name
                                                id
                                            }}
                                            runNumber
                                            updatedAt
                                            url
                                            databaseId
                                        }}
                                    }}
                                }}
                            }}
                        }}
                    }}
                    object(expression: "HEAD:.github/workflows") {{
                        ... on Tree {{
                            entries {{
                                name
                                object {{
                                    ... on Blob {{
                                        text
                                    }}
                                }}
                            }}
                        }}
                    }}
                }}
            }}
        """

        try:
            result = await self.execute_graphql(query, variables, use_cache=False)

            if not result.get("repository"):
                logger.warning("No data found for repository: %s", repo_name)
                return {"error": "Repository not found"}

            # Log summary of data collected
            repo_data = result.get("repository", {})
            pr_count = repo_data.get("pullRequests", {}).get("totalCount", 0)
            issue_count = repo_data.get("issues", {}).get("totalCount", 0)
            deployment_count = repo_data.get("deployments", {}).get("totalCount", 0)
            release_count = repo_data.get("releases", {}).get("totalCount", 0)

            # Check if we got actual nodes or just count information
            pr_nodes = len(repo_data.get("pullRequests", {}).get("nodes", []))
            issue_nodes = len(repo_data.get("issues", {}).get("nodes", []))
            deployment_nodes = len(repo_data.get("deployments", {}).get("nodes", []))

            logger.info(
                "Repository %s data summary - PRs: %d/%d, Issues: %d/%d, Deployments: %d/%d, Releases: %d",
                repo_name,
                pr_nodes,
                pr_count,
                issue_nodes,
                issue_count,
                deployment_nodes,
                deployment_count,
                release_count,
            )

            # If we have no data despite non-zero counts, log a warning
            if pr_count > 0 and pr_nodes == 0:
                logger.warning(
                    "Repository %s has %d PRs but no data was returned. This might be a permission issue.",
                    repo_name,
                    pr_count,
                )

            # Cache the result
            cache_key = self._cache_key(["repo_data", repo_name, str(days_lookback)])
            self._set_cache(cache_key, result)
            return result

        except Exception as e:
            logger.error("Failed to get data for repository %s: %s", repo_name, str(e))
            return {"error": str(e)}

    async def collect_dora_metrics(
        self, repos: List[str], output_file: Optional[Path] = None, batch_size: int = 5
    ):
        """
        Collect repository data for DORA metrics calculation with batching.
        This method only collects the data but does not compute the metrics.

        Args:
            repos: List of repository names
            output_file: Optional path to save results
            batch_size: Number of repositories to process in parallel

        Returns:
            Dictionary with collected data for all repositories
        """

        # Use the batch_process method for efficient processing
        async def process_repo(repo_name: str) -> Dict[str, Any]:
            return await self.get_repository_data(repo_name)

        logger.info(
            f"Collecting data for {len(repos)} repositories with batch size {batch_size}"
        )
        results = await self.batch_process(repos, process_repo, batch_size)

        # Process results
        data = {
            "organization": self.org_name,
            "collected_at": datetime.utcnow().isoformat(),
            "repositories": {},
        }

        for repo, result in zip(repos, results):
            if isinstance(result, Exception):
                logger.error(f"Failed to collect data for {repo}: {str(result)}")
                data["repositories"][repo] = {"error": str(result)}
            else:
                data["repositories"][repo] = result.get("repository", {})

        # Save results if output file specified
        if output_file:
            self._save_to_json(data, output_file)
            logger.info(f"Saved data to: {output_file}")

    def _save_to_json(self, data: Dict[str, Any], filepath: Path) -> None:
        """Save data to a JSON file"""
        try:
            filepath.parent.mkdir(parents=True, exist_ok=True)
            with open(filepath, "w") as f:
                json.dump(data, f, indent=2)
            logger.info(f"Saved data to: {filepath}")
        except Exception as e:
            logger.error(f"Failed to save data to {filepath}: {str(e)}")

    async def get_teams(self, include_members: bool = True) -> List[Dict[str, Any]]:
        """
        Get all teams in the organization with detailed information

        Args:
            include_members: Whether to include team members in the response

        Returns:
            List of team data dictionaries
        """
        # Try cache first
        cache_key = self._cache_key(["teams", str(include_members)])
        cached_data = self._get_from_cache(cache_key)
        if cached_data:
            return cached_data

        query = """
            query($org: String!, $cursor: String) {
                organization(login: $org) {
                    teams(first: 100, after: $cursor) {
                        pageInfo {
                            hasNextPage
                            endCursor
                        }
                        nodes {
                            id
                            name
                            slug
                            description
                            createdAt
                            updatedAt
                            privacy
                            repositories(first: 100) {
                                totalCount
                                nodes {
                                    name
                                    url
                                }
                            }
                            members(first: 100) @include(if: $includeMembers) {
                                totalCount
                                nodes {
                                    login
                                    name
                                    email
                                    company
                                    createdAt
                                    updatedAt
                                    location
                                    isHireable
                                    isSiteAdmin
                                    organizationVerifiedDomainEmails(login: $org)
                                }
                            }
                            childTeams(first: 100) {
                                totalCount
                                nodes {
                                    name
                                    slug
                                }
                            }
                            parentTeam {
                                name
                                slug
                            }
                        }
                    }
                }
            }
        """

        teams = []
        cursor = None

        try:
            while True:
                variables = {
                    "org": self.org_name,
                    "cursor": cursor,
                    "includeMembers": include_members,
                }
                result = await self.execute_graphql(query, variables)

                if not result.get("organization", {}).get("teams"):
                    logger.warning("Unexpected response format when fetching teams")
                    break

                page_info = result["organization"]["teams"]["pageInfo"]
                current_teams = result["organization"]["teams"]["nodes"]
                teams.extend(current_teams)

                if not page_info["hasNextPage"]:
                    break

                cursor = page_info["endCursor"]
                await self.check_rate_limit()

            # Cache the results
            if teams:
                self._set_cache(cache_key, teams)

            return teams

        except Exception as e:
            logger.error("Failed to get teams: %s", str(e))
            raise

    async def get_organization_members(
        self, include_detailed_info: bool = True
    ) -> List[Dict[str, Any]]:
        """
        Get all members of the organization with detailed contribution information

        Args:
            include_detailed_info: Whether to include detailed user information and contributions

        Returns:
            List of member data dictionaries
        """
        # Try cache first
        cache_key = self._cache_key(["org_members", str(include_detailed_info)])
        cached_data = self._get_from_cache(cache_key)
        if cached_data:
            return cached_data

        query = """
            query($org: String!, $cursor: String) {
                organization(login: $org) {
                    membersWithRole(first: 100, after: $cursor) {
                        pageInfo {
                            hasNextPage
                            endCursor
                        }
                        nodes {
                            login
                            name
                            email
                            company
                            createdAt
                            updatedAt
                            location
                            isHireable
                            isSiteAdmin
                            organizationVerifiedDomainEmails(login: $org)
                            contributionsCollection {
                                totalCommitContributions
                                totalIssueContributions
                                totalPullRequestContributions
                                totalPullRequestReviewContributions
                                commitContributionsByRepository(maxRepositories: 100) {
                                    repository {
                                        name
                                    }
                                    contributions {
                                        totalCount
                                    }
                                }
                                issueContributionsByRepository(maxRepositories: 100) {
                                    repository {
                                        name
                                    }
                                    contributions {
                                        totalCount
                                    }
                                }
                                pullRequestContributionsByRepository(maxRepositories: 100) {
                                    repository {
                                        name
                                    }
                                    contributions {
                                        totalCount
                                    }
                                }
                            }
                        }
                    }
                }
            }
        """

        members = []
        cursor = None

        try:
            while True:
                variables = {
                    "org": self.org_name,
                    "cursor": cursor,
                }
                result = await self.execute_graphql(query, variables)

                if not result.get("organization", {}).get("membersWithRole"):
                    logger.warning("Unexpected response format when fetching members")
                    break

                page_info = result["organization"]["membersWithRole"]["pageInfo"]
                current_members = result["organization"]["membersWithRole"]["nodes"]
                members.extend(current_members)

                if not page_info["hasNextPage"]:
                    break

                cursor = page_info["endCursor"]
                await self.check_rate_limit()

            # Cache the results
            if members:
                self._set_cache(cache_key, members)

            return members

        except Exception as e:
            logger.error("Failed to get organization members: %s", str(e))
            raise

    @classmethod
    def from_env(cls, org_name: str, **kwargs) -> "GitHubCollector":
        """
        Create a GitHubCollector from environment variables

        Args:
            org_name: GitHub organization name
            **kwargs: Override default parameters

        Returns:
            Configured GitHubCollector instance
        """
        # Get GitHub token from environment
        token = os.getenv("GITHUB_TOKEN")
        if not token:
            raise ValueError("GITHUB_TOKEN environment variable is required")

        # Get Redis configuration from environment if present
        use_redis = os.getenv("USE_REDIS", "").lower() in ("true", "1", "yes")

        # Default configuration
        config = {
            "token": token,
            "org_name": org_name,
            "use_redis": use_redis,
            "redis_host": os.getenv("REDIS_HOST", "localhost"),
            "redis_port": int(os.getenv("REDIS_PORT", "6379")),
            "redis_db": int(os.getenv("REDIS_DB", "0")),
        }
        logger.info("Config from env: %s", config)

        # Override with any kwargs provided
        config.update(kwargs)

        return cls(**config)
