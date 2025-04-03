use crate::common::types::PaginationParams;
use crate::domain::teams::{CreateTeamRequest, Team, TeamResult, UpdateTeamRequest};
use async_trait::async_trait;

#[async_trait]
pub trait TeamRepository {
    /// Creates a new team
    async fn create_team(&self, dto: CreateTeamRequest) -> TeamResult<Team>;

    /// Updates an existing team
    async fn update_team(&self, id: i32, dto: UpdateTeamRequest) -> TeamResult<Team>;

    /// Deletes a team
    async fn delete_team(&self, id: i32) -> TeamResult<bool>;

    /// Gets a team by its ID
    async fn get_team_by_id(&self, id: i32) -> TeamResult<Option<Team>>;

    /// Checks if a team exists
    async fn team_exists(&self, id: i32) -> TeamResult<bool>;

    /// Lists all teams for an organization with pagination
    async fn list_teams(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<Team>, i64)>;

    /// Lists all teams for a department with pagination
    async fn list_teams_by_department(
        &self,
        organization_id: i32,
        department_id: i32,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<Team>, i64)>;

    /// Searches for teams by name pattern
    async fn find_teams_by_name(
        &self,
        organization_id: i32,
        name_pattern: &str,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<Team>, i64)>;

    /// Gets multiple teams by their IDs
    async fn get_teams_by_ids(&self, ids: Vec<i32>) -> TeamResult<Vec<Team>>;

    /// Checks if a team name already exists in an organization
    async fn team_name_exists(&self, organization_id: i32, name: &str) -> TeamResult<bool>;

    /// Checks if a GitHub team name already exists in an organization
    async fn team_gh_name_exists(&self, organization_id: i32, name_gh: &str) -> TeamResult<bool>;

    /// Gets teams that a user is a member of
    async fn get_teams_by_user_id(
        &self,
        organization_id: i32,
        user_id: i32,
    ) -> TeamResult<Vec<Team>>;
}
