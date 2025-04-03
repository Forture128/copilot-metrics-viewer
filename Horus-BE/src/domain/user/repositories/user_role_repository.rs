use crate::common::types::PaginationParams;
use crate::domain::user::{UserResult, UserRole};
use async_trait::async_trait;

#[async_trait]
pub trait UserRoleRepository {
    /// Assigns a role to a user
    async fn assign_role(
        &self,
        organization_id: i32,
        user_id: i32,
        role_id: i32,
    ) -> UserResult<UserRole>;

    /// Removes a role from a user
    async fn remove_role(
        &self,
        organization_id: i32,
        user_id: i32,
        role_id: i32,
    ) -> UserResult<bool>;

    /// Gets all roles assigned to a user
    async fn get_user_roles(&self, organization_id: i32, user_id: i32)
        -> UserResult<Vec<UserRole>>;

    /// Gets all users with a specific role
    async fn get_users_with_role(
        &self,
        organization_id: i32,
        role_id: i32,
        params: &PaginationParams,
    ) -> UserResult<(Vec<i32>, i64)>;

    /// Checks if a user has a specific role
    async fn user_has_role(
        &self,
        organization_id: i32,
        user_id: i32,
        role_id: i32,
    ) -> UserResult<bool>;

    /// Gets all role IDs for a user
    async fn get_user_role_ids(&self, organization_id: i32, user_id: i32) -> UserResult<Vec<i32>>;

    /// Batch assigns roles to a user
    async fn batch_assign_roles(
        &self,
        organization_id: i32,
        user_id: i32,
        role_ids: Vec<i32>,
    ) -> UserResult<Vec<UserRole>>;

    /// Batch removes roles from a user
    async fn batch_remove_roles(
        &self,
        organization_id: i32,
        user_id: i32,
        role_ids: Vec<i32>,
    ) -> UserResult<usize>;

    /// Gets users with any of the specified roles
    async fn get_users_with_any_role(
        &self,
        organization_id: i32,
        role_ids: Vec<i32>,
        params: &PaginationParams,
    ) -> UserResult<(Vec<i32>, i64)>;

    /// Gets users with all of the specified roles
    async fn get_users_with_all_roles(
        &self,
        organization_id: i32,
        role_ids: Vec<i32>,
        params: &PaginationParams,
    ) -> UserResult<(Vec<i32>, i64)>;
}
