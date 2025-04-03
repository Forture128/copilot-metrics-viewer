"""
Collect comprehensive GitHub repository data using GraphQL API.
"""

import asyncio
from typing import Dict, Any, List
from src.common.github_graphql import GitHubGraphQLClient


class GitHubDataCollector:
    def __init__(self, token: str = None):
        self.client = GitHubGraphQLClient(token=token)

    async def get_repository_full_data(self, owner: str, name: str) -> Dict[str, Any]:
        """Get comprehensive repository data including issues, PRs, commits, etc."""
        query = """
        query($owner: String!, $name: String!) {
            repository(owner: $owner, name: $name) {
                # Basic Info
                id
                name
                nameWithOwner
                description
                url
                homepageUrl
                createdAt
                updatedAt
                pushedAt
                
                # Repository Settings
                isPrivate
                isArchived
                isFork
                isLocked
                isTemplate
                hasIssuesEnabled
                hasProjectsEnabled
                hasWikiEnabled
                hasDiscussionsEnabled
                
                # Statistics
                stargazerCount
                forkCount
                diskUsage
                
                # Code & Languages
                primaryLanguage {
                    name
                    color
                }
                languages(first: 100) {
                    totalCount
                    nodes {
                        name
                        color
                    }
                }
                
                # Branches & Tags
                refs(first: 100, refPrefix: "refs/heads/") {
                    nodes {
                        name
                        target {
                            ... on Commit {
                                history(first: 1) {
                                    totalCount
                                }
                            }
                        }
                    }
                }
                
                # Collaboration
                collaborators(first: 100) {
                    totalCount
                    nodes {
                        login
                        name
                        email
                        bio
                    }
                }
                
                # Issues
                issues(first: 100, orderBy: {field: CREATED_AT, direction: DESC}) {
                    totalCount
                    nodes {
                        number
                        title
                        state
                        createdAt
                        closedAt
                        author {
                            login
                        }
                        labels(first: 100) {
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
                
                # Pull Requests
                pullRequests(first: 100, orderBy: {field: CREATED_AT, direction: DESC}) {
                    totalCount
                    nodes {
                        number
                        title
                        state
                        createdAt
                        closedAt
                        mergedAt
                        author {
                            login
                        }
                        commits {
                            totalCount
                        }
                        additions
                        deletions
                        changedFiles
                    }
                }
                
                # Releases
                releases(first: 100, orderBy: {field: CREATED_AT, direction: DESC}) {
                    totalCount
                    nodes {
                        name
                        tagName
                        createdAt
                        isDraft
                        isPrerelease
                        author {
                            login
                        }
                    }
                }
                
                # Topics
                repositoryTopics(first: 100) {
                    nodes {
                        topic {
                            name
                        }
                    }
                }
                
                # Security
                vulnerabilityAlerts(first: 100) {
                    totalCount
                    nodes {
                        createdAt
                        dismissedAt
                        securityVulnerability {
                            severity
                            package {
                                name
                                ecosystem
                            }
                        }
                    }
                }
                
                # License
                licenseInfo {
                    name
                    spdxId
                    url
                }
            }
        }
        """
        response = await self.client.execute_query(
            query, {"owner": owner, "name": name}
        )
        return response.data

    async def get_repositories(self) -> List[Dict[str, str]]:
        """Get all repositories from the GitHub GraphQL API."""
        return await self.client.get_repositories()

    async def collect_repositories_data(
        self, repos: List[Dict[str, str]]
    ) -> List[Dict[str, Any]]:
        """Collect data for multiple repositories.

        Args:
            repos: List of dicts with 'owner' and 'name' keys
        """
        tasks = []
        for repo in repos:
            task = self.get_repository_full_data(repo["owner"], repo["name"])
            tasks.append(task)

        return await asyncio.gather(*tasks)


async def main():
    collector = GitHubDataCollector()
    repos = await collector.get_repositories()
    print(f"Found {len(repos)} repositories")
    print("Repositories: ", repos)
    if not repos:
        print("No repositories found")
        return
    # results = await collector.collect_repositories_data(repos)
    # for result in results:
    #     print(f"Collected data for: {result['repository']['nameWithOwner']}")


if __name__ == "__main__":
    asyncio.run(main())
