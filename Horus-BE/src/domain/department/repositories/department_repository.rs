use crate::common::types::PaginationParams;
use crate::domain::department::{
    CreateDepartmentRequest, Department, DepartmentResult, UpdateDepartmentRequest,
};
use async_trait::async_trait;

#[async_trait]
pub trait DepartmentRepository {
    /// Creates a new department
    async fn create_department(&self, dto: CreateDepartmentRequest)
        -> DepartmentResult<Department>;

    /// Updates an existing department
    async fn update_department(
        &self,
        id: i32,
        dto: UpdateDepartmentRequest,
    ) -> DepartmentResult<Department>;

    /// Deletes a department
    async fn delete_department(&self, id: i32) -> DepartmentResult<bool>;

    /// Gets a department by its ID
    async fn get_department_by_id(&self, id: i32) -> DepartmentResult<Option<Department>>;

    /// Checks if a department exists
    async fn department_exists(&self, id: i32) -> DepartmentResult<bool>;

    /// Lists all departments for an organization with pagination
    async fn list_departments(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> DepartmentResult<(Vec<Department>, i64)>;

    /// Searches for departments by name pattern
    async fn find_departments_by_name(
        &self,
        organization_id: i32,
        name_pattern: &str,
        params: &PaginationParams,
    ) -> DepartmentResult<(Vec<Department>, i64)>;

    /// Gets multiple departments by their IDs
    async fn get_departments_by_ids(&self, ids: Vec<i32>) -> DepartmentResult<Vec<Department>>;

    /// Checks if a department name already exists in an organization
    async fn department_name_exists(
        &self,
        organization_id: i32,
        name: &str,
    ) -> DepartmentResult<bool>;

    /// Lists all departments with pagination (admin only)
    async fn list_all_departments(
        &self,
        params: &PaginationParams,
    ) -> DepartmentResult<(Vec<Department>, i64)>;
}
