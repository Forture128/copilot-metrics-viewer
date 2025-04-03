"""
GitHub GraphQL Collector - Efficient data collection using GitHub's GraphQL API.

This module implements a GraphQL-based data collector that can efficiently fetch
data from GitHub's GraphQL API, which is often more efficient than REST API calls
for certain types of data collection.
"""

import os
import asyncio
from typing import Dict, List, Any, Optional, Union
import json
from pathlib import Path
import time
import logging

# Local imports
from src.common.github_graphql import GitHubGraphQLClient
from src.github_collector.collector import (
    CacheManager,
    logger,
    DEFAULT_CACHE_TTL,
    GitHubObjectEncoder,
)


class GitHubGraphQLCollector:
    """Collector for GitHub data using GraphQL API"""

    def __init__(
        self,
        token: Optional[str] = None,
        use_cache: bool = True,
        cache_ttl: int = DEFAULT_CACHE_TTL,
    ):
        # Load token from env if not provided
        self.token = token or os.getenv("GITHUB_TOKEN")
        if not self.token:
            raise ValueError(
                "GitHub token is required. Set GITHUB_TOKEN environment variable or pass token."
            )

        # Initialize GraphQL client
        self._graphql_client = GitHubGraphQLClient(token=self.token)

        # Setup caching
        self.use_cache = use_cache
        if use_cache:
            self.cache = CacheManager(ttl=cache_ttl)

    async def get_organization_overview(self, org_name: str) -> Dict[str, Any]:
        """
        Get organization overview information using GraphQL

        This function fetches basic information about the organization including:
        - Organization metadata
        - Member count
        - Repository count
        - Teams count
        """
        cache_key = f"graphql_org_overview_{org_name}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        query = """
        query($login: String!) {
            organization(login: $login) {
                login
                name
                description
                url
                websiteUrl
                avatarUrl
                twitterUsername
                email
                isVerified
                location
                createdAt
                updatedAt
                membersWithRole {
                    totalCount
                }
                repositories {
                    totalCount
                }
                teams {
                    totalCount
                }
                packages {
                    totalCount
                }
                projects(first: 0) {
                    totalCount
                }
                pinnedItems(first: 6) {
                    nodes {
                        ... on Repository {
                            name
                            description
                            url
                        }
                    }
                }
            }
        }
        """

        try:
            response = await self._graphql_client.execute_query(
                query, {"login": org_name}
            )
            result = response.data

            if self.use_cache:
                self.cache.set(cache_key, result)

            return result
        except Exception as e:
            logger.error(
                f"Error fetching organization overview for {org_name}: {str(e)}"
            )
            return {"error": str(e)}

    async def get_repositories_with_languages(
        self, org_name: str, first: int = 100
    ) -> Dict[str, Any]:
        """
        Get repositories with detailed language information

        This is much more efficient than making separate REST API calls for languages
        """
        cache_key = f"graphql_repos_languages_{org_name}_{first}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        query = """
        query($login: String!, $first: Int!) {
            organization(login: $login) {
                repositories(first: $first) {
                    totalCount
                    pageInfo {
                        hasNextPage
                        endCursor
                    }
                    nodes {
                        name
                        nameWithOwner
                        description
                        url
                        createdAt
                        updatedAt
                        pushedAt
                        isPrivate
                        isArchived
                        diskUsage
                        forkCount
                        stargazerCount
                        watchers {
                            totalCount
                        }
                        issues {
                            totalCount
                        }
                        pullRequests {
                            totalCount
                        }
                        defaultBranchRef {
                            name
                        }
                        languages(first: 10, orderBy: {field: SIZE, direction: DESC}) {
                            totalCount
                            totalSize
                            edges {
                                size
                                node {
                                    name
                                    color
                                }
                            }
                        }
                        licenseInfo {
                            name
                            spdxId
                        }
                    }
                }
            }
        }
        """

        try:
            response = await self._graphql_client.execute_query(
                query, {"login": org_name, "first": first}
            )
            result = response.data

            if self.use_cache:
                self.cache.set(cache_key, result)

            return result
        except Exception as e:
            logger.error(
                f"Error fetching repositories with languages for {org_name}: {str(e)}"
            )
            return {"error": str(e)}

    async def get_repository_collaborators(
        self, owner: str, name: str, first: int = 100
    ) -> Dict[str, Any]:
        """Get repository collaborators with permission information"""
        cache_key = f"graphql_repo_collabs_{owner}_{name}_{first}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        query = """
        query($owner: String!, $name: String!, $first: Int!) {
            repository(owner: $owner, name: $name) {
                collaborators(first: $first) {
                    totalCount
                    pageInfo {
                        hasNextPage
                        endCursor
                    }
                    edges {
                        permission
                        node {
                            login
                            name
                            email
                            avatarUrl
                            url
                        }
                    }
                }
            }
        }
        """

        try:
            response = await self._graphql_client.execute_query(
                query, {"owner": owner, "name": name, "first": first}
            )
            result = response.data

            if self.use_cache:
                self.cache.set(cache_key, result)

            return result
        except Exception as e:
            logger.error(f"Error fetching collaborators for {owner}/{name}: {str(e)}")
            return {"error": str(e)}

    async def get_repository_pull_requests(
        self,
        owner: str,
        name: str,
        first: int = 50,
        states: List[str] = ["OPEN", "CLOSED", "MERGED"],
    ) -> Dict[str, Any]:
        """Get repository pull requests with details"""
        states_str = str(states).replace("'", "").replace(" ", "")
        cache_key = f"graphql_repo_prs_{owner}_{name}_{first}_{states_str}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        query = """
        query($owner: String!, $name: String!, $first: Int!, $states: [PullRequestState!]) {
            repository(owner: $owner, name: $name) {
                pullRequests(first: $first, states: $states, orderBy: {field: CREATED_AT, direction: DESC}) {
                    totalCount
                    pageInfo {
                        hasNextPage
                        endCursor
                    }
                    nodes {
                        number
                        title
                        state
                        createdAt
                        updatedAt
                        closedAt
                        mergedAt
                        isDraft
                        url
                        author {
                            login
                        }
                        baseRefName
                        headRefName
                        additions
                        deletions
                        changedFiles
                        commits(first: 1) {
                            totalCount
                        }
                        reviews(first: 10) {
                            totalCount
                            nodes {
                                author {
                                    login
                                }
                                state
                                submittedAt
                            }
                        }
                        reviewRequests(first: 5) {
                            totalCount
                            nodes {
                                requestedReviewer {
                                    ... on User {
                                        login
                                    }
                                    ... on Team {
                                        name
                                    }
                                }
                            }
                        }
                        labels(first: 10) {
                            nodes {
                                name
                                color
                            }
                        }
                    }
                }
            }
        }
        """

        try:
            response = await self._graphql_client.execute_query(
                query, {"owner": owner, "name": name, "first": first, "states": states}
            )
            result = response.data

            if self.use_cache:
                self.cache.set(cache_key, result)

            return result
        except Exception as e:
            logger.error(f"Error fetching pull requests for {owner}/{name}: {str(e)}")
            return {"error": str(e)}

    async def get_repository_issues(
        self,
        owner: str,
        name: str,
        first: int = 50,
        states: List[str] = ["OPEN", "CLOSED"],
    ) -> Dict[str, Any]:
        """Get repository issues with details"""
        states_str = str(states).replace("'", "").replace(" ", "")
        cache_key = f"graphql_repo_issues_{owner}_{name}_{first}_{states_str}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        query = """
        query($owner: String!, $name: String!, $first: Int!, $states: [IssueState!]) {
            repository(owner: $owner, name: $name) {
                issues(first: $first, states: $states, orderBy: {field: CREATED_AT, direction: DESC}) {
                    totalCount
                    pageInfo {
                        hasNextPage
                        endCursor
                    }
                    nodes {
                        number
                        title
                        state
                        createdAt
                        updatedAt
                        closedAt
                        url
                        author {
                            login
                        }
                        assignees(first: 5) {
                            nodes {
                                login
                            }
                        }
                        labels(first: 10) {
                            nodes {
                                name
                                color
                            }
                        }
                        comments {
                            totalCount
                        }
                    }
                }
            }
        }
        """

        try:
            response = await self._graphql_client.execute_query(
                query, {"owner": owner, "name": name, "first": first, "states": states}
            )
            result = response.data

            if self.use_cache:
                self.cache.set(cache_key, result)

            return result
        except Exception as e:
            logger.error(f"Error fetching issues for {owner}/{name}: {str(e)}")
            return {"error": str(e)}

    async def get_organization_teams_with_members(
        self, org_name: str, first_teams: int = 100, first_members: int = 100
    ) -> Dict[str, Any]:
        """Get organization teams with their members"""
        cache_key = f"graphql_org_teams_{org_name}_{first_teams}_{first_members}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        query = """
        query($login: String!, $firstTeams: Int!, $firstMembers: Int!) {
            organization(login: $login) {
                teams(first: $firstTeams) {
                    totalCount
                    pageInfo {
                        hasNextPage
                        endCursor
                    }
                    nodes {
                        id
                        name
                        slug
                        description
                        privacy
                        url
                        createdAt
                        updatedAt
                        members(first: $firstMembers) {
                            totalCount
                            pageInfo {
                                hasNextPage
                                endCursor
                            }
                            nodes {
                                login
                                name
                                email
                                avatarUrl
                            }
                        }
                        repositories(first: 10) {
                            totalCount
                            nodes {
                                name
                                nameWithOwner
                            }
                        }
                    }
                }
            }
        }
        """

        try:
            response = await self._graphql_client.execute_query(
                query,
                {
                    "login": org_name,
                    "firstTeams": first_teams,
                    "firstMembers": first_members,
                },
            )
            result = response.data

            if self.use_cache:
                self.cache.set(cache_key, result)

            return result
        except Exception as e:
            logger.error(f"Error fetching teams with members for {org_name}: {str(e)}")
            return {"error": str(e)}

    async def collect_organization_data(self, org_name: str) -> Dict[str, Any]:
        """
        Collect comprehensive organization data using GraphQL API

        This collects:
        - Organization overview
        - Repositories with languages
        - Teams with members

        GraphQL allows collecting this data more efficiently than multiple REST API calls.
        """
        logger.info(f"Collecting GraphQL data for organization: {org_name}")

        # Collect data concurrently for better performance
        overview_task = self.get_organization_overview(org_name)
        repos_task = self.get_repositories_with_languages(org_name)
        teams_task = self.get_organization_teams_with_members(org_name)

        # Wait for all tasks to complete
        overview, repos, teams = await asyncio.gather(
            overview_task, repos_task, teams_task, return_exceptions=True
        )

        # Process results
        result = {
            "organization": org_name,
            "timestamp": time.time(),
            "overview": overview
            if not isinstance(overview, Exception)
            else {"error": str(overview)},
            "repositories": repos
            if not isinstance(repos, Exception)
            else {"error": str(repos)},
            "teams": teams
            if not isinstance(teams, Exception)
            else {"error": str(teams)},
        }

        # Extract repository names for detailed data collection
        repo_names = []
        if not isinstance(repos, Exception) and "organization" in repos:
            org_repos = repos["organization"]["repositories"]["nodes"]
            repo_names = [(org_name, repo["name"]) for repo in org_repos]

        # If we have repositories, collect additional data for a subset (to avoid rate limits)
        # Limit to first 10 repos for detailed data
        detailed_repos = repo_names[:10] if repo_names else []

        # Collect detailed repository data concurrently
        repo_details = {}
        if detailed_repos:
            logger.info(
                f"Collecting detailed GraphQL data for {len(detailed_repos)} repositories"
            )

            tasks = []
            for owner, name in detailed_repos:
                tasks.append(self.get_repository_collaborators(owner, name))
                tasks.append(self.get_repository_pull_requests(owner, name))
                tasks.append(self.get_repository_issues(owner, name))

            # Process in batches to avoid overwhelming the API
            batch_size = 3  # Process 1 repo at a time (3 calls per repo)
            for i in range(0, len(tasks), batch_size):
                batch_tasks = tasks[i : i + batch_size]
                batch_results = await asyncio.gather(
                    *batch_tasks, return_exceptions=True
                )

                # Add results to repo_details
                repo_idx = i // batch_size
                if repo_idx < len(detailed_repos):
                    owner, name = detailed_repos[repo_idx]
                    repo_key = f"{owner}/{name}"

                    repo_details[repo_key] = {
                        "collaborators": batch_results[0]
                        if not isinstance(batch_results[0], Exception)
                        else {"error": str(batch_results[0])},
                        "pull_requests": batch_results[1]
                        if not isinstance(batch_results[1], Exception)
                        else {"error": str(batch_results[1])},
                        "issues": batch_results[2]
                        if not isinstance(batch_results[2], Exception)
                        else {"error": str(batch_results[2])},
                    }

                # Short pause to avoid hitting rate limits
                await asyncio.sleep(1)

        result["repository_details"] = repo_details

        logger.info(f"GraphQL data collection complete for {org_name}")
        return result

    def collect_organization_data_sync(self, org_name: str) -> Dict[str, Any]:
        """Synchronous wrapper for collect_organization_data"""
        loop = asyncio.new_event_loop()
        try:
            return loop.run_until_complete(self.collect_organization_data(org_name))
        finally:
            loop.close()

    def save_data_to_json(self, data: Dict[str, Any], filepath: str) -> None:
        """Save collected data to a JSON file"""
        try:
            # Ensure directory exists
            Path(filepath).parent.mkdir(parents=True, exist_ok=True)

            # Save data
            with open(filepath, "w") as f:
                json.dump(data, f, indent=2, cls=GitHubObjectEncoder)

            logger.info(f"Data saved to {filepath}")
        except Exception as e:
            logger.error(f"Error saving data to {filepath}: {str(e)}")
            raise


def main():
    """Main function to demonstrate GitHub GraphQL collector usage"""
    import argparse

    parser = argparse.ArgumentParser(description="GitHub GraphQL Data Collector")
    parser.add_argument("--org", required=True, help="GitHub organization name")
    parser.add_argument(
        "--token", help="GitHub token (optional, will use env var if not provided)"
    )
    parser.add_argument(
        "--output", default="data/github_graphql_data.json", help="Output file path"
    )
    parser.add_argument("--no-cache", action="store_true", help="Disable caching")
    args = parser.parse_args()

    # Initialize collector
    collector = GitHubGraphQLCollector(token=args.token, use_cache=not args.no_cache)

    # Collect data
    data = collector.collect_organization_data_sync(args.org)

    # Save data
    collector.save_data_to_json(data, args.output)

    print(f"GitHub GraphQL data collected and saved to {args.output}")


if __name__ == "__main__":
    main()
