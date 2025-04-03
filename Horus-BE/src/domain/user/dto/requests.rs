use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

/// Request to create a new user
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    /// User's organization ID
    pub organization_id: i32,

    /// Username (must be unique)
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: String,

    /// Email address (must be unique)
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,

    /// Password (stored as a hash)
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

/// Request to update an existing user
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    /// Updated username (optional)
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: Option<String>,

    /// Updated email (optional)
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: Option<String>,
}

/// Request to change a user's password
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    /// Current password for verification
    pub current_password: String,

    /// New password to set
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub new_password: String,
}

/// Request to authenticate a user
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    /// Username or email for login
    pub username_or_email: String,

    /// Password for authentication
    pub password: String,
}

/// Request to assign a role to a user
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AssignRoleRequest {
    /// User ID to assign the role to
    pub user_id: i32,

    /// Role ID to assign
    pub role_id: i32,
}
