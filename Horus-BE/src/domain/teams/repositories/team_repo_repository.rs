use crate::common::types::PaginationParams;
use crate::domain::teams::{TeamRepo, TeamResult};
use async_trait::async_trait;

#[async_trait]
pub trait TeamRepoRepository {
    /// Assigns a repository to a team
    async fn assign_repo_to_team(
        &self,
        organization_id: i32,
        team_id: i32,
        repo_id: i32,
    ) -> TeamResult<TeamRepo>;

    /// Removes a repository from a team
    async fn remove_repo_from_team(
        &self,
        organization_id: i32,
        team_id: i32,
        repo_id: i32,
    ) -> TeamResult<bool>;

    /// Gets all repositories assigned to a team
    async fn get_team_repos(
        &self,
        organization_id: i32,
        team_id: i32,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<TeamRepo>, i64)>;

    /// Gets all team IDs a repository is assigned to
    async fn get_repo_team_ids(&self, organization_id: i32, repo_id: i32) -> TeamResult<Vec<i32>>;

    /// Checks if a repository is assigned to a team
    async fn is_repo_assigned_to_team(
        &self,
        organization_id: i32,
        team_id: i32,
        repo_id: i32,
    ) -> TeamResult<bool>;

    /// Batch assigns repositories to a team
    async fn batch_assign_repos_to_team(
        &self,
        organization_id: i32,
        team_id: i32,
        repo_ids: Vec<i32>,
    ) -> TeamResult<Vec<TeamRepo>>;

    /// Batch removes repositories from a team
    async fn batch_remove_repos_from_team(
        &self,
        organization_id: i32,
        team_id: i32,
        repo_ids: Vec<i32>,
    ) -> TeamResult<usize>;

    /// Gets all repository IDs for a team
    async fn get_team_repo_ids(&self, organization_id: i32, team_id: i32) -> TeamResult<Vec<i32>>;

    /// Counts repositories assigned to a team
    async fn count_team_repos(&self, organization_id: i32, team_id: i32) -> TeamResult<i64>;
}
