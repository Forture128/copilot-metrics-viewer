use crate::common::types::PaginationParams;
use crate::domain::teams::{TeamMember, TeamResult};
use async_trait::async_trait;

#[async_trait]
pub trait TeamMemberRepository {
    /// Adds a user to a team
    async fn add_team_member(
        &self,
        organization_id: i32,
        team_id: i32,
        user_id: i32,
    ) -> TeamResult<TeamMember>;

    /// Removes a user from a team
    async fn remove_team_member(
        &self,
        organization_id: i32,
        team_id: i32,
        user_id: i32,
    ) -> TeamResult<bool>;

    /// Gets all members of a team
    async fn get_team_members(
        &self,
        organization_id: i32,
        team_id: i32,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<TeamMember>, i64)>;

    /// Gets all team IDs a user belongs to
    async fn get_user_team_ids(&self, organization_id: i32, user_id: i32) -> TeamResult<Vec<i32>>;

    /// Checks if a user is a member of a team
    async fn is_team_member(
        &self,
        organization_id: i32,
        team_id: i32,
        user_id: i32,
    ) -> TeamResult<bool>;

    /// Batch adds users to a team
    async fn batch_add_team_members(
        &self,
        organization_id: i32,
        team_id: i32,
        user_ids: Vec<i32>,
    ) -> TeamResult<Vec<TeamMember>>;

    /// Batch removes users from a team
    async fn batch_remove_team_members(
        &self,
        organization_id: i32,
        team_id: i32,
        user_ids: Vec<i32>,
    ) -> TeamResult<usize>;

    /// Gets all user IDs for a team
    async fn get_team_user_ids(&self, organization_id: i32, team_id: i32) -> TeamResult<Vec<i32>>;

    /// Counts members in a team
    async fn count_team_members(&self, organization_id: i32, team_id: i32) -> TeamResult<i64>;
}
