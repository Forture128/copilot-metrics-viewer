use crate::common::types::PaginationParams;
use crate::domain::roles::{AssignDepartmentRoleRequest, DepartmentRole, RoleResult};
use async_trait::async_trait;

#[async_trait]
pub trait DepartmentRoleRepository {
    /// Assigns a role to a user within a department
    async fn assign_department_role(
        &self,
        organization_id: i32,
        request: AssignDepartmentRoleRequest,
    ) -> RoleResult<DepartmentRole>;

    /// Removes a role from a user within a department
    async fn remove_department_role(
        &self,
        organization_id: i32,
        department_id: i32,
        user_id: i32,
        role_id: i32,
    ) -> RoleResult<bool>;

    /// Gets all department roles for a user
    async fn get_user_department_roles(
        &self,
        organization_id: i32,
        user_id: i32,
    ) -> RoleResult<Vec<DepartmentRole>>;

    /// Gets all users with a specific role in a department
    async fn get_users_with_department_role(
        &self,
        organization_id: i32,
        department_id: i32,
        role_id: i32,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<i32>, i64)>;

    /// Checks if a user has a specific role in a department
    async fn user_has_department_role(
        &self,
        organization_id: i32,
        department_id: i32,
        user_id: i32,
        role_id: i32,
    ) -> RoleResult<bool>;

    /// Gets all department role IDs for a user
    async fn get_user_department_role_ids(
        &self,
        organization_id: i32,
        user_id: i32,
        department_id: i32,
    ) -> RoleResult<Vec<i32>>;

    /// Gets all users with any of the specified roles in a department
    async fn get_users_with_any_department_role(
        &self,
        organization_id: i32,
        department_id: i32,
        role_ids: Vec<i32>,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<i32>, i64)>;

    /// Gets all department roles for a department
    async fn get_department_roles(
        &self,
        organization_id: i32,
        department_id: i32,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<DepartmentRole>, i64)>;

    /// Batch assigns department roles
    async fn batch_assign_department_roles(
        &self,
        organization_id: i32,
        requests: Vec<AssignDepartmentRoleRequest>,
    ) -> RoleResult<Vec<DepartmentRole>>;

    /// Batch removes department roles
    async fn batch_remove_department_roles(
        &self,
        organization_id: i32,
        department_id: i32,
        user_ids: Vec<i32>,
        role_id: i32,
    ) -> RoleResult<usize>;
}
