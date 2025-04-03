use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::common::types::PaginationParams;
use crate::domain::teams::{
    AddTeamMemberRequest, AssignRepoToTeamRequest, BatchAddTeamMembersRequest,
    BatchAssignReposToTeamRequest, CreateTeamRequest, Team, TeamError, TeamMember,
    TeamMemberRepository, TeamRepo, TeamRepoRepository, TeamRepository, TeamResult,
    TeamWithMembersResponse, TeamWithReposResponse, UpdateTeamRequest,
};
use crate::domain::user::{User, UserRepository, UserResponse};
use crate::infrastructure::{database::BaseRepository, redis::Redis};

/// Service for managing teams, team members, and team repositories
pub struct TeamService<R, MR, RR, UR>
where
    R: TeamRepository + BaseRepository<Entity = Team>,
    MR: TeamMemberRepository,
    RR: TeamRepoRepository,
    UR: UserRepository,
{
    repository: R,
    member_repository: MR,
    repo_repository: RR,
    user_repository: UR,
    redis: Redis,
    cache: Arc<RwLock<HashMap<i32, Team>>>,
}

impl<R, MR, RR, UR> TeamService<R, MR, RR, UR>
where
    R: TeamRepository + BaseRepository<Entity = Team>,
    MR: TeamMemberRepository,
    RR: TeamRepoRepository,
    UR: UserRepository,
{
    /// Creates a new instance of the team service
    pub fn new(
        repository: R,
        member_repository: MR,
        repo_repository: RR,
        user_repository: UR,
        redis: Redis,
    ) -> Self {
        Self {
            repository,
            member_repository,
            repo_repository,
            user_repository,
            redis,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new team
    pub async fn create_team(&self, request: CreateTeamRequest) -> TeamResult<Team> {
        // Check if team name already exists in the organization
        if self
            .repository
            .team_name_exists(request.organization_id, &request.name)
            .await?
        {
            return Err(TeamError::TeamAlreadyExists);
        }

        // Check if GitHub team name already exists in the organization
        if self
            .repository
            .team_gh_name_exists(request.organization_id, &request.name_gh)
            .await?
        {
            return Err(TeamError::TeamAlreadyExists);
        }

        let team = self.repository.create_team(request).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.insert(team.id, team.clone());

        Ok(team)
    }

    /// Updates an existing team
    pub async fn update_team(&self, id: i32, request: UpdateTeamRequest) -> TeamResult<Team> {
        // Check if team exists
        let existing = self.repository.get_team_by_id(id).await?;
        if existing.is_none() {
            return Err(TeamError::TeamNotFound(id));
        }
        let existing = existing.unwrap();

        // Check name uniqueness if it's being updated
        if let Some(ref name) = request.name {
            if self
                .repository
                .team_name_exists(existing.organization_id, name)
                .await?
                && existing.name != *name
            {
                return Err(TeamError::TeamAlreadyExists);
            }
        }

        // Check GitHub name uniqueness if it's being updated
        if let Some(ref name_gh) = request.name_gh {
            if self
                .repository
                .team_gh_name_exists(existing.organization_id, name_gh)
                .await?
                && existing.name_gh != *name_gh
            {
                return Err(TeamError::TeamAlreadyExists);
            }
        }

        let team = self.repository.update_team(id, request).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.insert(team.id, team.clone());

        Ok(team)
    }

    /// Deletes a team
    pub async fn delete_team(&self, id: i32) -> TeamResult<bool> {
        // Check if team exists
        let existing = self.repository.get_team_by_id(id).await?;
        if existing.is_none() {
            return Err(TeamError::TeamNotFound(id));
        }

        let result = self.repository.delete_team(id).await?;

        // Update cache if successful
        if result {
            let mut cache = self.cache.write().await;
            cache.remove(&id);
        }

        Ok(result)
    }

    /// Gets a team by ID
    pub async fn get_team(&self, id: i32) -> TeamResult<Option<Team>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(team) = cache.get(&id) {
                return Ok(Some(team.clone()));
            }
        }

        // If not in cache, get from repository
        let team = self.repository.get_team_by_id(id).await?;

        // Update cache if team found
        if let Some(ref team) = team {
            let mut cache = self.cache.write().await;
            cache.insert(team.id, team.clone());
        }

        Ok(team)
    }

    /// Lists teams with pagination
    pub async fn list_teams(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<Team>, i64)> {
        self.repository.list_teams(organization_id, params).await
    }

    /// Lists teams by department with pagination
    pub async fn list_teams_by_department(
        &self,
        organization_id: i32,
        department_id: i32,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<Team>, i64)> {
        self.repository
            .list_teams_by_department(organization_id, department_id, params)
            .await
    }

    /// Adds a member to a team
    pub async fn add_team_member(
        &self,
        team_id: i32,
        request: AddTeamMemberRequest,
    ) -> TeamResult<TeamMember> {
        // Check if team exists and get the organization_id
        let team = match self.get_team(team_id).await? {
            Some(team) => team,
            None => return Err(TeamError::TeamNotFound(team_id)),
        };

        // Check if user exists
        let user_exists = self
            .user_repository
            .get_user_by_id(request.user_id)
            .await
            .map_err(|e| TeamError::InternalError(format!("Failed to check user: {}", e)))?
            .is_some();

        if !user_exists {
            return Err(TeamError::UserNotFound(request.user_id));
        }

        // Check if user is already a member
        if self
            .member_repository
            .is_team_member(team.organization_id, team_id, request.user_id)
            .await?
        {
            return Err(TeamError::UserAlreadyInTeam(request.user_id, team_id));
        }

        // Add the member
        self.member_repository
            .add_team_member(team.organization_id, team_id, request.user_id)
            .await
    }

    /// Removes a member from a team
    pub async fn remove_team_member(&self, team_id: i32, user_id: i32) -> TeamResult<bool> {
        // Check if team exists and get the organization_id
        let team = match self.get_team(team_id).await? {
            Some(team) => team,
            None => return Err(TeamError::TeamNotFound(team_id)),
        };

        // Check if user is a member
        if !self
            .member_repository
            .is_team_member(team.organization_id, team_id, user_id)
            .await?
        {
            return Err(TeamError::UserNotInTeam(user_id, team_id));
        }

        // Remove the member
        self.member_repository
            .remove_team_member(team.organization_id, team_id, user_id)
            .await
    }

    /// Gets a team with its members
    pub async fn get_team_with_members(
        &self,
        team_id: i32,
        params: &PaginationParams,
    ) -> TeamResult<TeamWithMembersResponse> {
        // Get the team
        let team = match self.get_team(team_id).await? {
            Some(team) => team,
            None => return Err(TeamError::TeamNotFound(team_id)),
        };

        // Get the members
        let (team_members, _) = self
            .member_repository
            .get_team_members(team.organization_id, team_id, params)
            .await?;

        // Get user IDs
        let user_ids = team_members
            .iter()
            .map(|member| member.user_id)
            .collect::<Vec<_>>();

        // Get users
        let users = self
            .user_repository
            .get_users_by_ids(user_ids)
            .await
            .map_err(|e| TeamError::InternalError(format!("Failed to get users: {}", e)))?;

        // Convert to user responses
        let user_responses = users
            .iter()
            .map(|user| self.to_user_response(user))
            .collect();

        Ok(TeamWithMembersResponse {
            team: self.to_team_response(&team),
            members: user_responses,
        })
    }

    /// Batch adds members to a team
    pub async fn batch_add_team_members(
        &self,
        team_id: i32,
        request: BatchAddTeamMembersRequest,
    ) -> TeamResult<Vec<TeamMember>> {
        // Check if team exists and get the organization_id
        let team = match self.get_team(team_id).await? {
            Some(team) => team,
            None => return Err(TeamError::TeamNotFound(team_id)),
        };

        // Add the members
        self.member_repository
            .batch_add_team_members(team.organization_id, team_id, request.user_ids)
            .await
    }

    /// Assigns a repository to a team
    pub async fn assign_repo_to_team(
        &self,
        team_id: i32,
        request: AssignRepoToTeamRequest,
    ) -> TeamResult<TeamRepo> {
        // Check if team exists and get the organization_id
        let team = match self.get_team(team_id).await? {
            Some(team) => team,
            None => return Err(TeamError::TeamNotFound(team_id)),
        };

        // Check if repo is already assigned
        if self
            .repo_repository
            .is_repo_assigned_to_team(team.organization_id, team_id, request.repo_id)
            .await?
        {
            return Err(TeamError::RepositoryError(format!(
                "Repository {} is already assigned to team {}",
                request.repo_id, team_id
            )));
        }

        // Assign the repo
        self.repo_repository
            .assign_repo_to_team(team.organization_id, team_id, request.repo_id)
            .await
    }

    /// Removes a repository from a team
    pub async fn remove_repo_from_team(&self, team_id: i32, repo_id: i32) -> TeamResult<bool> {
        // Check if team exists and get the organization_id
        let team = match self.get_team(team_id).await? {
            Some(team) => team,
            None => return Err(TeamError::TeamNotFound(team_id)),
        };

        // Check if repo is assigned
        if !self
            .repo_repository
            .is_repo_assigned_to_team(team.organization_id, team_id, repo_id)
            .await?
        {
            return Err(TeamError::RepositoryError(format!(
                "Repository {} is not assigned to team {}",
                repo_id, team_id
            )));
        }

        // Remove the repo
        self.repo_repository
            .remove_repo_from_team(team.organization_id, team_id, repo_id)
            .await
    }

    /// Gets a team with its repositories
    pub async fn get_team_with_repos(&self, team_id: i32) -> TeamResult<TeamWithReposResponse> {
        // Get the team
        let team = match self.get_team(team_id).await? {
            Some(team) => team,
            None => return Err(TeamError::TeamNotFound(team_id)),
        };

        // Get the repository IDs
        let repo_ids = self
            .repo_repository
            .get_team_repo_ids(team.organization_id, team_id)
            .await?;

        Ok(TeamWithReposResponse {
            team: self.to_team_response(&team),
            repo_ids,
        })
    }

    /// Batch assigns repositories to a team
    pub async fn batch_assign_repos_to_team(
        &self,
        team_id: i32,
        request: BatchAssignReposToTeamRequest,
    ) -> TeamResult<Vec<TeamRepo>> {
        // Check if team exists and get the organization_id
        let team = match self.get_team(team_id).await? {
            Some(team) => team,
            None => return Err(TeamError::TeamNotFound(team_id)),
        };

        // Assign the repos
        self.repo_repository
            .batch_assign_repos_to_team(team.organization_id, team_id, request.repo_ids)
            .await
    }

    /// Converts a Team entity to a TeamResponse DTO
    fn to_team_response(&self, team: &Team) -> crate::domain::teams::TeamResponse {
        crate::domain::teams::TeamResponse {
            id: team.id,
            organization_id: team.organization_id,
            department_id: team.department_id,
            name: team.name.clone(),
            name_gh: team.name_gh.clone(),
            created_at: team.created_at,
            updated_at: team.updated_at,
        }
    }

    /// Converts a User entity to a UserResponse DTO
    fn to_user_response(&self, user: &User) -> UserResponse {
        UserResponse {
            id: user.id,
            organization_id: user.organization_id,
            username: user.username.clone(),
            email: user.email.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
