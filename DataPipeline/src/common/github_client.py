from abc import ABC, abstractmethod
from typing import Optional, List, Dict, Any, Union
from datetime import datetime
from github import Github, Auth, Organization
from github.PaginatedList import PaginatedList
from github.Repository import Repository
from github.Team import Team
from github.NamedUser import NamedUser
from github.PullRequest import PullRequest
from github.Deployment import Deployment
from github.WorkflowRun import WorkflowRun
from github.Issue import Issue
from github.Commit import Commit
from github.GithubObject import (
    NotSet,
    Opt,
)
from dataclasses import dataclass

from src.common.exceptions import (
    GithubClientException,
    OrganizationServiceException,
)


@dataclass
class PaginatedResponse:
    """Class to handle paginated responses"""

    items: List[any]
    total_count: int
    page: int
    per_page: int
    has_next: bool


class GithubClientInterface(ABC):
    """Abstract base class defining the interface for Github operations"""

    @abstractmethod
    def connect(self) -> None:
        """Establish connection to Github API"""
        pass

    @abstractmethod
    def get_organization(self, org_name: str) -> Organization:
        """Get organization details"""
        pass


class GithubClient(GithubClientInterface):
    """Concrete implementation of Github client using PyGithub"""

    def __init__(self, token: str):
        self._token = token
        self._client: Optional[Github] = None
        self._organization: Optional[Any] = None

    def connect(self) -> None:
        """Initialize Github client with authentication"""
        try:
            self._client = Github(auth=Auth.Token(self._token))
        except Exception as e:
            raise GithubClientException(f"Failed to connect to Github: {str(e)}") from e

    @property
    def client(self) -> Github:
        """Get Github client instance, connecting if necessary"""
        if not self._client:
            self.connect()
        return self._client

    def get_organization(self, org_name: str) -> Organization:
        """Get organization by name"""
        try:
            self._organization = self.client.get_organization(org_name)
            return self._organization
        except Exception as e:
            raise GithubClientException(
                f"Failed to get organization {org_name}: {str(e)}"
            ) from e

    @property
    def organization(self) -> Organization:
        """Get current organization instance"""
        if not self._organization:
            raise GithubClientException(
                "No organization selected. Call get_organization() first."
            )
        return self._organization


class OrganizationService:
    """Service class for organization-specific operations"""

    def __init__(self, github_client: GithubClient):
        self._github = github_client

    def get_teams(self) -> PaginatedList[Team]:
        """Get all teams in the organization"""
        try:
            return self._github.organization.get_teams()
        except Exception as e:
            raise OrganizationServiceException(f"Failed to get teams: {str(e)}") from e

    def get_repositories(
        self,
        type: Opt[str] = NotSet,
        sort: Opt[str] = NotSet,
        direction: Opt[str] = NotSet,
    ) -> PaginatedList[Repository]:
        """Get all repositories in the organization with optional filtering and sorting
        :calls: `GET /orgs/{org}/repos <https://docs.github.com/en/rest/reference/repos>`_
        :param type: string ('all', 'public', 'private', 'forks', 'sources', 'member')
        :param sort: string ('created', 'updated', 'pushed', 'full_name')
        :param direction: string ('asc', desc')
        """
        try:
            return self._github.organization.get_repos(
                type=type,
                sort=sort,
                direction=direction,
            )
        except Exception as e:
            raise OrganizationServiceException(
                f"Failed to get repositories: {str(e)}"
            ) from e

    def get_members(self) -> PaginatedList[NamedUser]:
        """Get all members in the organization"""
        try:
            return self._github.organization.get_members()
        except Exception as e:
            raise OrganizationServiceException(
                f"Failed to get members: {str(e)}"
            ) from e

    def get_team_members(self, team_id: int) -> PaginatedList[NamedUser]:
        """Get members of a specific team"""
        try:
            team = self._github.organization.get_team(team_id)
            return team.get_members()
        except Exception as e:
            raise OrganizationServiceException(
                f"Failed to get team members: {str(e)}"
            ) from e


class DoraModuleService:
    """Service class for DORA metrics related operations"""

    def __init__(self, github_client: GithubClient):
        self._github = github_client

    def get_deployments(self, owner: str, repo: str) -> List[Deployment]:
        """Get deployments for a repository"""
        try:
            repository = self._github.client.get_repo(f"{owner}/{repo}")
            return list(repository.get_deployments())
        except Exception as e:
            raise GithubClientException(f"Failed to get deployments: {str(e)}") from e

    def get_pull_requests(
        self, owner: str, repo: str, params: Dict[str, Any] = None
    ) -> List[PullRequest]:
        """Get pull requests for a repository"""
        try:
            repository = self._github.client.get_repo(f"{owner}/{repo}")
            state = params.get("state", "all") if params else "all"
            base = params.get("base", None) if params else None
            pulls = (
                repository.get_pulls(state=state, base=base)
                if base
                else repository.get_pulls(state=state)
            )
            return list(pulls)
        except Exception as e:
            raise GithubClientException(f"Failed to get pull requests: {str(e)}") from e

    def get_deployment_statuses(
        self, owner: str, repo: str, deployment_id: int
    ) -> List[Dict[str, Any]]:
        """Get deployment statuses for a specific deployment"""
        try:
            repository = self._github.client.get_repo(f"{owner}/{repo}")
            deployment = repository.get_deployment(deployment_id)
            return list(deployment.get_statuses())
        except Exception as e:
            raise GithubClientException(
                f"Failed to get deployment statuses: {str(e)}"
            ) from e

    def get_commits(self, owner: str, repo: str) -> List[Commit]:
        """Get commits for a repository"""
        try:
            repository = self._github.client.get_repo(f"{owner}/{repo}")
            return list(repository.get_commits())
        except Exception as e:
            raise GithubClientException(f"Failed to get commits: {str(e)}") from e

    def get_workflow_runs(self, owner: str, repo: str) -> List[WorkflowRun]:
        """Get workflow runs for a repository"""
        try:
            repository = self._github.client.get_repo(f"{owner}/{repo}")
            return list(repository.get_workflow_runs())
        except Exception as e:
            raise GithubClientException(f"Failed to get workflow runs: {str(e)}") from e

    def get_issues(self, owner: str, repo: str) -> List[Issue]:
        """Get issues for a repository"""
        try:
            repository = self._github.client.get_repo(f"{owner}/{repo}")
            return list(repository.get_issues())
        except Exception as e:
            raise GithubClientException(f"Failed to get issues: {str(e)}") from e

    def get_time_pull_request_reviews(
        self, owner: str, repo: str, pull_number: int
    ) -> Optional[Dict[str, datetime]]:
        """Get review timeline information for a pull request"""
        try:
            repository = self._github.client.get_repo(f"{owner}/{repo}")
            pull_request = repository.get_pull(pull_number)
            reviews = list(pull_request.get_reviews())

            if not reviews:
                return None

            first_review = reviews[0]
            approved_review = next(
                (review for review in reviews if review.state == "APPROVED"), None
            )
            last_review = reviews[-1]

            return {
                "first_review_at": first_review.submitted_at,
                "approved_at": approved_review.submitted_at
                if approved_review
                else None,
                "review_completed_at": last_review.submitted_at,
            }
        except Exception as e:
            raise GithubClientException(
                f"Failed to get PR review timeline: {str(e)}"
            ) from e

    def get_repository(self, owner: str, repo: str) -> Repository:
        """Get repository information"""
        try:
            return self._github.client.get_repo(f"{owner}/{repo}")
        except Exception as e:
            raise GithubClientException(f"Failed to get repository: {str(e)}") from e

    def get_repositories(self, page: int = 1, per_page: int = 50) -> List[Repository]:
        """Get list of repositories with pagination"""
        try:
            user = self._github.client.get_user()
            repos = user.get_repos(sort="updated")
            # Calculate start and end indices for pagination
            start = (page - 1) * per_page
            end = start + per_page
            return list(repos[start:end])
        except Exception as e:
            raise GithubClientException(f"Failed to get repositories: {str(e)}") from e


class GithubClientFactory:
    """Factory class for creating Github clients"""

    @staticmethod
    def create(token: str) -> GithubClient:
        client = GithubClient(token)
        client.connect()  # Validate connection immediately
        return client
