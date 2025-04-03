use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response representing a user
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    /// User ID
    pub id: i32,
    /// Organization ID
    pub organization_id: i32,
    /// Username
    pub username: String,
    /// Email address
    pub email: String,
    /// When the user was created
    pub created_at: NaiveDateTime,
    /// When the user was last updated
    pub updated_at: NaiveDateTime,
}

/// Response for user authentication
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    /// Access token for API authentication
    pub access_token: String,
    /// Token type (usually "Bearer")
    pub token_type: String,
    /// Token expiration time in seconds
    pub expires_in: u64,
    /// Basic user information
    pub user: UserResponse,
}

/// Response for user with roles
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserWithRolesResponse {
    /// User information
    pub user: UserResponse,
    /// List of role IDs assigned to the user
    pub role_ids: Vec<i32>,
}

/// Response with a list of users
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserListResponse {
    /// List of users
    pub users: Vec<UserResponse>,
    /// Total count of users (for pagination)
    pub total: i64,
    /// Current page
    pub page: i64,
    /// Page size
    pub page_size: i64,
}

/// Response for user creation or update
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserActionResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// User data if operation was successful
    pub user: Option<UserResponse>,
    /// Error message if operation failed
    pub message: Option<String>,
}
