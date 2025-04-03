"""
Extended GitHub Collector - Additional collectors for GitHub data beyond the base collector.

This module extends the base GitHubCollector with additional collectors for:
- Repository workflows and actions
- Team management
- Releases
- Deployment environments
- Projects
- Code scanning and security alerts
"""

import asyncio
from typing import Dict, List, Any
from datetime import datetime

# PyGithub imports
from github.Repository import Repository
from github.GithubException import RateLimitExceededException
from github.Team import Team
from github.WorkflowRun import WorkflowRun
from github.Workflow import Workflow
from github.GitRelease import GitRelease
from github.Issue import Issue
from github.PullRequest import PullRequest
from github.Commit import Commit

# from github.ProjectCard import ProjectCard
# from github.ProjectColumn import ProjectColumn
from github.Organization import Organization
from github.NamedUser import NamedUser
from github.PaginatedList import PaginatedList
from github.Deployment import Deployment
from github.Branch import Branch
from github.Project import Project
from github.BranchProtection import BranchProtection

# Local imports
from src.github_collector.collector import (
    GitHubCollector,
    retry_on_exception,
    rate_limit_aware,
    logger,
)


class ExtendedGitHubCollector(GitHubCollector):
    """Extended collector with additional GitHub API functionality"""

    @rate_limit_aware()
    @retry_on_exception(exceptions=(RateLimitExceededException, Exception))
    def get_repository_workflows(
        self, owner: str, repo: str
    ) -> PaginatedList[Workflow] | list[Workflow]:
        """Get all workflows for a repository"""
        cache_key = f"workflows_{owner}_{repo}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        repository: Repository = self._github_client.client.get_repo(f"{owner}/{repo}")

        try:
            workflows: PaginatedList[Workflow] = list(repository.get_workflows())
            if self.use_cache:
                self.cache.set(cache_key, workflows)

            return workflows
        except Exception as e:
            logger.warning(f"Failed to get workflows for {owner}/{repo}: {str(e)}")
            return []

    @rate_limit_aware()
    @retry_on_exception(exceptions=(RateLimitExceededException, Exception))
    def get_workflow_runs(
        self, owner: str, repo: str, workflow_id: int
    ) -> PaginatedList[WorkflowRun] | list[WorkflowRun]:
        """Get recent workflow runs for a specific workflow"""
        cache_key = f"workflow_runs_{owner}_{repo}_{workflow_id}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        repository: Repository = self._github_client.client.get_repo(f"{owner}/{repo}")
        workflow: Workflow = repository.get_workflow(workflow_id)

        try:
            runs: PaginatedList[WorkflowRun] = list(workflow.get_runs())[:20]
            if self.use_cache:
                self.cache.set(cache_key, runs)

            return runs
        except Exception as e:
            logger.warning(
                f"Failed to get workflow runs for {owner}/{repo}/{workflow_id}: {str(e)}"
            )
            return []

    @rate_limit_aware()
    @retry_on_exception(exceptions=(RateLimitExceededException, Exception))
    def get_teams(self, org_name: str) -> List[Team]:
        """Get all teams in an organization"""
        cache_key = f"teams_{org_name}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        teams: PaginatedList[Team] = self.org.get_teams()
        if self.use_cache:
            self.cache.set(cache_key, teams)

        # TEST: Return only the first team
        return teams

    @rate_limit_aware()
    @retry_on_exception(exceptions=(RateLimitExceededException, Exception))
    def get_team_members(
        self, org_name: str, team_slug: str
    ) -> PaginatedList[NamedUser] | list[NamedUser]:
        """Get all members of a team"""
        cache_key = f"team_members_{org_name}_{team_slug}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        team: Team = self.org.get_team_by_slug(team_slug)
        members: PaginatedList[NamedUser] = list(team.get_members())

        if self.use_cache:
            self.cache.set(cache_key, members)

        return members

    @rate_limit_aware()
    @retry_on_exception(exceptions=(RateLimitExceededException, Exception))
    def get_releases(
        self, owner: str, repo: str
    ) -> PaginatedList[GitRelease] | list[GitRelease]:
        """Get all releases for a repository"""
        cache_key = f"releases_{owner}_{repo}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        repository: Repository = self._github_client.client.get_repo(f"{owner}/{repo}")
        releases: PaginatedList[GitRelease] = list(repository.get_releases())
        if self.use_cache:
            self.cache.set(cache_key, releases)

        return releases

    @rate_limit_aware()
    @retry_on_exception(exceptions=(RateLimitExceededException, Exception))
    def get_deployments(
        self, owner: str, repo: str
    ) -> PaginatedList[Deployment] | list[Deployment]:
        """Get all deployments for a repository"""
        cache_key = f"deployments_{owner}_{repo}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        repository: Repository = self._github_client.client.get_repo(f"{owner}/{repo}")

        try:
            deployments: PaginatedList[Deployment] = list(repository.get_deployments())
            if self.use_cache:
                self.cache.set(cache_key, deployments)

            return deployments
        except Exception as e:
            logger.warning(f"Failed to get deployments for {owner}/{repo}: {str(e)}")
            return []

    @rate_limit_aware()
    @retry_on_exception(exceptions=(RateLimitExceededException, Exception))
    def get_repository_branches(self, owner: str, repo: str) -> List[Branch]:
        """Get all branches for a repository"""
        cache_key = f"branches_{owner}_{repo}"
        if self.use_cache:
            cached_data = self.cache.get(cache_key)
            if cached_data:
                return cached_data

        repository: Repository = self._github_client.client.get_repo(f"{owner}/{repo}")
        branches: list[Branch] = list(repository.get_branches())

        if self.use_cache:
            self.cache.set(cache_key, branches)

        return branches

    async def collect_extended_data(self, org_name: str) -> Dict[str, Any]:
        """
        Collect extended data from GitHub for an organization

        This collects additional data beyond the base collector:
        - Teams and team members
        - Workflows and workflow runs
        - Repository branches and protection
        - Releases and assets
        - Deployments and deployment statuses
        - Projects and project boards
        """
        # First, collect base data
        base_data = await self.collect_all_data(org_name)

        # Now collect extended data
        logger.info(f"Collecting extended data for organization: {org_name}")

        # Start with empty extended data structure
        extended_data = {
            "teams": {},
            "workflows": {},
            "branches": {},
            "releases": {},
            "deployments": {},
            "projects": {},
        }

        # Collect teams and team members
        try:
            teams: list[Team] = self.get_teams(org_name)
            print("[collect_extended_data] teams = ", teams)
            extended_data["teams"]["list"] = teams

            # Collect team members for each team
            team_members = {}
            for team in teams:
                try:
                    team_slug = team.slug
                    members: list[NamedUser] = self.get_team_members(
                        org_name, team_slug
                    )
                    team_members[team_slug] = members
                except Exception as e:
                    logger.error(
                        f"Error collecting team members for {team.name}: {str(e)}"
                    )
                    team_members[team_slug] = []

            extended_data["teams"]["members"] = team_members
        except Exception as e:
            logger.error(f"Error collecting teams for {org_name}: {str(e)}")
            extended_data["teams"] = {"list": [], "members": {}}

        # Collect repository-specific extended data concurrently
        repositories: list[Repository] = base_data["repositories"]

        # Helper functions for concurrent collection
        async def collect_workflow_data():
            workflows_data: dict[str, list[Workflow]] = {}
            workflow_runs_data: dict[str, dict[str, list[WorkflowRun]]] = {}

            for repo in repositories:
                repo_name = repo.name
                try:
                    workflows: list[Workflow] = self.get_repository_workflows(
                        org_name, repo_name
                    )
                    workflows_data[repo_name] = workflows

                    # Collect workflow runs for each workflow
                    runs_by_workflow: dict[str, list[WorkflowRun]] = {}
                    for workflow in workflows:
                        try:
                            workflow_id = workflow.id
                            runs: list[WorkflowRun] = self.get_workflow_runs(
                                org_name, repo_name, workflow_id
                            )
                            runs_by_workflow[workflow_id] = runs
                        except Exception as e:
                            logger.error(
                                f"Error collecting workflow runs for {repo_name}/{workflow.name}: {str(e)}"
                            )
                            runs_by_workflow[workflow_id] = []

                    workflow_runs_data[repo_name] = runs_by_workflow

                except Exception as e:
                    logger.error(
                        f"Error collecting workflows for {repo_name}: {str(e)}"
                    )
                    workflows_data[repo_name] = []
                    workflow_runs_data[repo_name] = {}

            extended_data["workflows"]["list"] = workflows_data
            extended_data["workflows"]["runs"] = workflow_runs_data

        async def collect_branch_data():
            branches_data: dict[str, list[Branch]] = {}

            for repo in repositories:
                repo_name = repo.name
                try:
                    branches: list[Branch] = self.get_repository_branches(
                        org_name, repo_name
                    )
                    branches_data[repo_name] = branches
                except Exception as e:
                    logger.error(f"Error collecting branches for {repo_name}: {str(e)}")
                    branches_data[repo_name] = []

            extended_data["branches"] = branches_data

        async def collect_release_data():
            releases_data: dict[str, list[GitRelease]] = {}

            for repo in repositories:
                repo_name = repo.name
                try:
                    releases: list[GitRelease] = self.get_releases(org_name, repo_name)
                    releases_data[repo_name] = releases
                except Exception as e:
                    logger.error(f"Error collecting releases for {repo_name}: {str(e)}")
                    releases_data[repo_name] = []

            extended_data["releases"] = releases_data

        async def collect_deployment_data():
            deployments_data: dict[str, list[Deployment]] = {}

            for repo in repositories:
                repo_name = repo.name
                try:
                    deployments: list[Deployment] = self.get_deployments(
                        org_name, repo_name
                    )
                    deployments_data[repo_name] = deployments
                except Exception as e:
                    logger.error(
                        f"Error collecting deployments for {repo_name}: {str(e)}"
                    )
                    deployments_data[repo_name] = []

            extended_data["deployments"] = deployments_data

        # Run all data collection concurrently
        collection_tasks = [
            collect_workflow_data(),
            collect_branch_data(),
            collect_release_data(),
            collect_deployment_data(),
        ]

        await asyncio.gather(*collection_tasks, return_exceptions=True)

        logger.info(f"Extended data collection complete for {org_name}")

        # Combine base and extended data
        combined_data = {**base_data}
        combined_data["extended"] = extended_data

        return combined_data

    def collect_extended_data_sync(self, org_name: str) -> Dict[str, Any]:
        """Synchronous wrapper for collect_extended_data"""
        loop = asyncio.new_event_loop()
        try:
            return loop.run_until_complete(self.collect_extended_data(org_name))
        finally:
            loop.close()

    async def collect_extended_data_async(self, org_name: str) -> Dict[str, Any]:
        """Asynchronously collect extended GitHub data for an organization"""
        logger.info(
            f"[EXTENDED] Starting async data collection for organization: {org_name}"
        )

        try:
            # Get repositories
            repos = await self._get_repositories_async()
            logger.info(f"[EXTENDED] Found {len(repos)} repositories")

            # Collect data for each repository concurrently
            tasks = []
            for repo in repos:
                tasks.append(self._collect_repository_data_async(repo))

            # Wait for all tasks to complete
            results = await asyncio.gather(*tasks, return_exceptions=True)

            # Process results
            processed_results = []
            for result in results:
                if isinstance(result, Exception):
                    logger.error(
                        f"[EXTENDED] Error collecting repository data: {str(result)}"
                    )
                    continue
                processed_results.append(result)

            logger.info(
                f"[EXTENDED] Successfully collected data for {len(processed_results)} repositories"
            )

            return {
                "organization": org_name,
                "repositories": processed_results,
                "metadata": {
                    "timestamp": datetime.now().isoformat(),
                    "total_repositories": len(repos),
                    "successful_collections": len(processed_results),
                },
            }

        except Exception as e:
            logger.error(f"[EXTENDED] Failed to collect extended data: {str(e)}")
            raise

    async def _get_repositories_async(self) -> List[Repository]:
        """Asynchronously get repositories for an organization"""
        return list(self.org.get_repos())

    async def _collect_repository_data_async(self, repo: Repository) -> Dict[str, Any]:
        """Asynchronously collect detailed data for a repository"""
        try:
            logger.info(f"[EXTENDED] Collecting data for repository: {repo.full_name}")

            # Collect data concurrently
            contributors_task = self._get_contributors_async(repo)
            issues_task = self._get_issues_async(repo)
            pull_requests_task = self._get_pull_requests_async(repo)
            commits_task = self._get_commits_async(repo)

            # Wait for all tasks to complete
            contributors, issues, pull_requests, commits = await asyncio.gather(
                contributors_task,
                issues_task,
                pull_requests_task,
                commits_task,
                return_exceptions=True,
            )

            # Process results
            processed_data = {
                "contributors": contributors
                if not isinstance(contributors, Exception)
                else [],
                "issues": issues if not isinstance(issues, Exception) else [],
                "pull_requests": pull_requests
                if not isinstance(pull_requests, Exception)
                else [],
                "commits": commits if not isinstance(commits, Exception) else [],
            }

            # Combine results
            repo_data = repo
            repo_data.update(processed_data)

            logger.info(
                f"[EXTENDED] Successfully collected data for repository: {repo.full_name}"
            )
            return repo_data

        except Exception as e:
            logger.error(
                f"[EXTENDED] Error collecting data for repository {repo.full_name}: {str(e)}"
            )
            # Return basic repo data if collection fails
            return repo

    async def _get_contributors_async(self, repository: Repository) -> List[NamedUser]:
        """Asynchronously get contributors for a repository"""
        return list(repository.get_contributors())

    async def _get_issues_async(self, repository: Repository) -> List[Issue]:
        """Asynchronously get issues for a repository"""
        return list(repository.get_issues(state="all"))

    async def _get_pull_requests_async(
        self, repository: Repository
    ) -> List[PullRequest]:
        """Asynchronously get pull requests for a repository"""
        return list(repository.get_pulls(state="all"))

    async def _get_commits_async(self, repository: Repository) -> List[Commit]:
        """Asynchronously get commits for a repository"""
        return list(repository.get_commits())


def main():
    """Main function to demonstrate extended GitHub collector usage"""
    import argparse

    parser = argparse.ArgumentParser(description="Extended GitHub Data Collector")
    parser.add_argument("--org", required=True, help="GitHub organization name")
    parser.add_argument(
        "--token", help="GitHub token (optional, will use env var if not provided)"
    )
    parser.add_argument(
        "--output", default="data/github_extended_data.json", help="Output file path"
    )
    parser.add_argument(
        "--concurrent", type=int, default=5, help="Number of concurrent requests"
    )
    parser.add_argument("--no-cache", action="store_true", help="Disable caching")
    args = parser.parse_args()

    # Initialize collector
    collector = ExtendedGitHubCollector(
        token=args.token,
        concurrent_requests=args.concurrent,
        use_cache=not args.no_cache,
    )

    # Collect data
    data = collector.collect_extended_data_sync(args.org)

    # Save data
    collector.save_data_to_json(data, args.output)

    print(f"Extended GitHub data collected and saved to {args.output}")


if __name__ == "__main__":
    main()
