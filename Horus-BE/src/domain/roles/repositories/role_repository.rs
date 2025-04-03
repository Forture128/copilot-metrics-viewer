use crate::common::types::PaginationParams;
use crate::domain::roles::{CreateRoleRequest, Role, RoleResult, UpdateRoleRequest};
use async_trait::async_trait;

#[async_trait]
pub trait RoleRepository {
    /// Creates a new role
    async fn create_role(&self, dto: CreateRoleRequest) -> RoleResult<Role>;

    /// Updates an existing role
    async fn update_role(&self, id: i32, dto: UpdateRoleRequest) -> RoleResult<Role>;

    /// Deletes a role
    async fn delete_role(&self, id: i32) -> RoleResult<bool>;

    /// Gets a role by its ID
    async fn get_role_by_id(&self, id: i32) -> RoleResult<Option<Role>>;

    /// Checks if a role exists
    async fn role_exists(&self, id: i32) -> RoleResult<bool>;

    /// Lists all roles for an organization with pagination
    async fn list_roles(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<Role>, i64)>;

    /// Searches for roles by name pattern
    async fn find_roles_by_name(
        &self,
        organization_id: i32,
        name_pattern: &str,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<Role>, i64)>;

    /// Gets multiple roles by their IDs
    async fn get_roles_by_ids(&self, ids: Vec<i32>) -> RoleResult<Vec<Role>>;

    /// Checks if a role name already exists in an organization
    async fn role_name_exists(&self, organization_id: i32, name: &str) -> RoleResult<bool>;

    /// Creates multiple roles in a batch operation
    async fn batch_create_roles(
        &self,
        organization_id: i32,
        roles: Vec<CreateRoleRequest>,
    ) -> RoleResult<Vec<Role>>;

    /// Gets all roles assigned to a user across the system
    async fn get_roles_by_user_id(
        &self,
        organization_id: i32,
        user_id: i32,
    ) -> RoleResult<Vec<Role>>;
}
