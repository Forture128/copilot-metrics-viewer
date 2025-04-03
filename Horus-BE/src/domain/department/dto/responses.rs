use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response representing a department
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DepartmentResponse {
    /// Department ID
    pub id: i32,
    /// Organization ID
    pub organization_id: i32,
    /// Department name
    pub name: String,
    /// When the department was created
    pub created_at: NaiveDateTime,
    /// When the department was last updated
    pub updated_at: NaiveDateTime,
}

/// Response for a list of departments
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DepartmentListResponse {
    /// List of departments
    pub departments: Vec<DepartmentResponse>,
    /// Total count of departments (for pagination)
    pub total: i64,
    /// Current page
    pub page: i64,
    /// Page size
    pub page_size: i64,
}

/// Response for department creation or update
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DepartmentActionResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Department data if operation was successful
    pub department: Option<DepartmentResponse>,
    /// Error message if operation failed
    pub message: Option<String>,
}

/// Response for a department with its users
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DepartmentWithUsersResponse {
    /// Department information
    pub department: DepartmentResponse,
    /// List of user IDs in the department
    pub user_ids: Vec<i32>,
}

/// Response for a department membership
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DepartmentMembershipResponse {
    /// User ID
    pub user_id: i32,
    /// Department ID
    pub department_id: i32,
    /// When the user was added to the department
    pub joined_at: NaiveDateTime,
}
