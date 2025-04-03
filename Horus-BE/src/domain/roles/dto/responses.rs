use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::roles::{DepartmentRole, Role};

/// Response representing a role
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RoleResponse {
    /// Role ID
    pub id: i32,
    /// Organization ID
    pub organization_id: i32,
    /// Role name
    pub name: String,
    /// Role description
    pub description: Option<String>,
    /// When the role was created
    pub created_at: NaiveDateTime,
    /// When the role was last updated
    pub updated_at: NaiveDateTime,
}

/// Response representing a department role assignment
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DepartmentRoleResponse {
    /// Department role ID
    pub id: i32,
    /// Department ID
    pub department_id: i32,
    /// User ID
    pub user_id: i32,
    /// Role ID
    pub role_id: i32,
}

/// Response for a list of roles
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RoleListResponse {
    /// List of roles
    pub roles: Vec<RoleResponse>,
    /// Total count of roles (for pagination)
    pub total: i64,
    /// Current page
    pub page: i64,
    /// Page size
    pub page_size: i64,
}

/// Response for role creation or update
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RoleActionResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Role data if operation was successful
    pub role: Option<RoleResponse>,
    /// Error message if operation failed
    pub message: Option<String>,
}

/// Response for batch role creation
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BatchRoleActionResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// List of created/updated roles
    pub roles: Vec<RoleResponse>,
    /// Count of successful operations
    pub success_count: usize,
    /// Error message if any
    pub message: Option<String>,
}

// Conversion implementations
impl From<Role> for RoleResponse {
    fn from(role: Role) -> Self {
        Self {
            id: role.id,
            organization_id: role.organization_id,
            name: role.name,
            description: role.description,
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}

impl From<DepartmentRole> for DepartmentRoleResponse {
    fn from(department_role: DepartmentRole) -> Self {
        Self {
            id: department_role.id,
            department_id: department_role.department_id,
            user_id: department_role.user_id,
            role_id: department_role.role_id,
        }
    }
}
