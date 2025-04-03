use crate::common::types::PaginationParams;
use crate::domain::department::DepartmentResult;
use crate::schema::department_users;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a department-user association
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = department_users)]
pub struct DepartmentUser {
    /// Unique identifier for the department-user association
    pub id: i32,
    /// ID of the organization
    pub organization_id: i32,
    /// ID of the department
    pub department_id: i32,
    /// ID of the user
    pub user_id: i32,
    /// When the user was added to the department
    pub joined_at: NaiveDateTime,
}

#[async_trait]
pub trait DepartmentUserRepository {
    /// Adds a user to a department
    async fn add_user_to_department(
        &self,
        organization_id: i32,
        department_id: i32,
        user_id: i32,
    ) -> DepartmentResult<DepartmentUser>;

    /// Removes a user from a department
    async fn remove_user_from_department(
        &self,
        organization_id: i32,
        department_id: i32,
        user_id: i32,
    ) -> DepartmentResult<bool>;

    /// Gets all users in a department
    async fn get_department_users(
        &self,
        organization_id: i32,
        department_id: i32,
        params: &PaginationParams,
    ) -> DepartmentResult<(Vec<DepartmentUser>, i64)>;

    /// Gets all department IDs a user belongs to
    async fn get_user_department_ids(
        &self,
        organization_id: i32,
        user_id: i32,
    ) -> DepartmentResult<Vec<i32>>;

    /// Checks if a user is in a department
    async fn is_user_in_department(
        &self,
        organization_id: i32,
        department_id: i32,
        user_id: i32,
    ) -> DepartmentResult<bool>;

    /// Batch adds users to a department
    async fn batch_add_users_to_department(
        &self,
        organization_id: i32,
        department_id: i32,
        user_ids: Vec<i32>,
    ) -> DepartmentResult<Vec<DepartmentUser>>;

    /// Batch removes users from a department
    async fn batch_remove_users_from_department(
        &self,
        organization_id: i32,
        department_id: i32,
        user_ids: Vec<i32>,
    ) -> DepartmentResult<usize>;

    /// Gets all user IDs for a department
    async fn get_department_user_ids(
        &self,
        organization_id: i32,
        department_id: i32,
    ) -> DepartmentResult<Vec<i32>>;

    /// Counts users in a department
    async fn count_department_users(
        &self,
        organization_id: i32,
        department_id: i32,
    ) -> DepartmentResult<i64>;
}
