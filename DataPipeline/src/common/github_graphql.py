"""
GitHub GraphQL API client and utilities for data pipeline using aiographql.
"""

import os
from typing import Dict, Any, Optional
import asyncio
from aiographql.client import GraphQLClient, GraphQLResponse
from graphql import build_ast_schema, parse
from dotenv import load_dotenv


class GitHubGraphQLClient:
    def __init__(self, token: Optional[str] = None, schema_path: Optional[str] = None):
        """Initialize GitHub GraphQL client.

        Args:
            token: GitHub Personal Access Token. If not provided, will try to get from environment.
            schema_path: Path to GraphQL schema file. If provided, will load schema from file.
        """
        load_dotenv()
        self.token = token or os.getenv("GITHUB_TOKEN")
        if not self.token:
            raise ValueError(
                "GitHub token is required. Set GITHUB_TOKEN environment variable or pass token directly."
            )

        # Initialize async GraphQL client
        self.client = GraphQLClient(
            endpoint="https://api.github.com/graphql",
            headers={"Authorization": f"Bearer {self.token}"},
        )

        # Load schema if provided
        if schema_path:
            with open(schema_path, "r", encoding="utf-8") as schema_file:
                schema_str = schema_file.read()
            schema_ast = parse(schema_str)
            self.schema = build_ast_schema(schema_ast)
            self.client.schema = self.schema

    async def execute_query(
        self, query: str, variables: Optional[Dict[str, Any]] = None
    ) -> GraphQLResponse:
        """Execute a GraphQL query asynchronously.

        Args:
            query: GraphQL query string
            variables: Optional variables for the query

        Returns:
            GraphQLResponse containing the query results
        """
        try:
            return await self.client.query(query, variables or {})
        except Exception as e:
            raise Exception(f"Failed to execute GraphQL query: {str(e)}")

    async def get_repository_info(self, owner: str, name: str) -> Dict[str, Any]:
        """Get comprehensive repository information asynchronously.

        Args:
            owner: Repository owner
            name: Repository name

        Returns:
            Dict containing repository information
        """
        query = """
        query($owner: String!, $name: String!) {
            repository(owner: $owner, name: $name) {
                id
                name
                nameWithOwner
                description
                url
                homepageUrl
                createdAt
                updatedAt
                pushedAt
                isFork
                isArchived
                isDisabled
                isLocked
                isPrivate
                hasIssuesEnabled
                hasWikiEnabled
                hasDiscussionsEnabled
                primaryLanguage {
                    name
                    color
                }
                languages(first: 10) {
                    nodes {
                        name
                        color
                    }
                }
                defaultBranchRef {
                    name
                    target {
                        ... on Commit {
                            oid
                            committedDate
                        }
                    }
                }
                stargazerCount
                forkCount
                watchers {
                    totalCount
                }
                issues {
                    totalCount
                }
                pullRequests {
                    totalCount
                }
                diskUsage
                licenseInfo {
                    name
                    spdxId
                }
            }
        }
        """
        response = await self.execute_query(query, {"owner": owner, "name": name})
        return response.data

    def get_repository_commits(
        self, owner: str, name: str, branch: str = "main", limit: int = 100
    ) -> Dict[str, Any]:
        """Get commit history for a repository.

        Args:
            owner: Repository owner
            name: Repository name
            branch: Branch name (default: main)
            limit: Number of commits to fetch

        Returns:
            Dict containing commit information
        """
        query = """
        query($owner: String!, $name: String!, $branch: String!, $limit: Int!) {
            repository(owner: $owner, name: $name) {
                ref(qualifiedName: $branch) {
                    target {
                        ... on Commit {
                            history(first: $limit) {
                                nodes {
                                    oid
                                    messageHeadline
                                    committedDate
                                    author {
                                        name
                                        email
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        """
        variables = {"owner": owner, "name": name, "branch": branch, "limit": limit}
        return self.execute_query(query, variables)

    async def get_schema_info(self) -> Dict[str, Any]:
        """Get information about available types and fields from the schema."""
        query = """
        query IntrospectionQuery {
            __schema {
                types {
                    name
                    description
                    fields {
                        name
                        description
                        type {
                            name
                        }
                    }
                }
            }
        }
        """
        response = await self.execute_query(query)
        return response.data

    async def validate_field_exists(self, type_name: str, field_name: str) -> bool:
        """Check if a field exists on a type in the schema.

        Args:
            type_name: Name of the type (e.g., 'Repository')
            field_name: Name of the field to check
        """
        if not self.schema:
            raise ValueError("Schema not loaded. Initialize client with schema_path")

        type_def = self.schema.get_type(type_name)
        if not type_def:
            return False

        fields = type_def.fields
        return field_name in fields

    async def get_repositories(self) -> Dict[str, Any]:
        """Get all repositories asynchronously."""
        query = """
        query {
            search(query: "is:public", type: REPOSITORY, first: 100) {
                nodes {
                    ... on Repository {
                        id
                        name
                        nameWithOwner
                        description
                        url
                        homepageUrl
                        createdAt
                        updatedAt
                        pushedAt
                        isFork
                    }
                }
            }
        }
        """
        response = await self.execute_query(query)
        return response.data

    async def get_teams(self) -> Dict[str, Any]:
        """Get all teams asynchronously."""
        query = """
        query {
            search(query: "is:team", type: TEAM, first: 100) {
                nodes {
                    ... on Team {
                        id
                        name
                        slug
                        description
                        url
                        members {
                            totalCount
                        }
                    }
                }
            }
        """
        response = await self.execute_query(query)
        return response.data
