use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Request to create a new role
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateRoleRequest {
    /// Organization ID the role belongs to
    pub organization_id: i32,

    /// Name of the role
    #[validate(length(
        min = 2,
        max = 100,
        message = "Role name must be between 2 and 100 characters"
    ))]
    pub name: String,

    /// Description of the role (optional)
    #[validate(length(max = 500, message = "Role description must not exceed 500 characters"))]
    pub description: Option<String>,
}

/// Request to update an existing role
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateRoleRequest {
    /// Updated name (optional)
    #[validate(length(
        min = 2,
        max = 100,
        message = "Role name must be between 2 and 100 characters"
    ))]
    pub name: Option<String>,

    /// Updated description (optional)
    #[validate(length(max = 500, message = "Role description must not exceed 500 characters"))]
    pub description: Option<String>,
}

/// Request to assign a department role
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AssignDepartmentRoleRequest {
    /// User ID to assign the department role to
    pub user_id: i32,

    /// Department ID for the role assignment
    pub department_id: i32,

    /// Role ID to assign
    pub role_id: i32,
}

/// Request to batch create roles
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct BatchCreateRolesRequest {
    /// Organization ID the roles belong to
    pub organization_id: i32,

    /// List of role names to create
    #[validate(length(min = 1, message = "At least one role must be provided"))]
    pub roles: Vec<CreateRoleRequest>,
}
