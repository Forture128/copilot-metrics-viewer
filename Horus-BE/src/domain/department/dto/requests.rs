use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Request to create a new department
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateDepartmentRequest {
    /// Organization ID the department belongs to
    pub organization_id: i32,

    /// Name of the department
    #[validate(length(
        min = 2,
        max = 100,
        message = "Department name must be between 2 and 100 characters"
    ))]
    pub name: String,
}

/// Request to update an existing department
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateDepartmentRequest {
    /// Updated name (optional)
    #[validate(length(
        min = 2,
        max = 100,
        message = "Department name must be between 2 and 100 characters"
    ))]
    pub name: Option<String>,
}

/// Request to assign a user to a department
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AssignUserToDepartmentRequest {
    /// User ID to assign to the department
    pub user_id: i32,
}

/// Request to batch add users to a department
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct BatchAssignUsersToDepartmentRequest {
    /// List of user IDs to assign to the department
    #[validate(length(min = 1, message = "At least one user ID must be provided"))]
    pub user_ids: Vec<i32>,
}
