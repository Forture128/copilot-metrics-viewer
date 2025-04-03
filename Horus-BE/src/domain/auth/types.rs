use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request for logging in to the system
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

/// Response when authentication is successful
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user_id: i32,
    pub organization_id: i32,
    pub is_super_admin: bool,
}

/// User information response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserInfoResponse {
    pub user_id: i32,
    pub organization_id: i32,
    pub role: String,
}

/// Common error response for authentication failures
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthErrorResponse {
    pub error: String,
}

/// Authentication result type
pub type AuthResult<T> = Result<T, AuthError>;

/// Authentication error
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Token generation error: {0}")]
    TokenError(String),

    #[error("Invalid or expired token")]
    InvalidToken,

    #[error("Unauthorized access")]
    Unauthorized,
}

impl From<AuthError> for actix_web::HttpResponse {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::InvalidCredentials => {
                actix_web::HttpResponse::Unauthorized().json(AuthErrorResponse {
                    error: "Invalid credentials".to_string(),
                })
            }
            AuthError::DatabaseError(msg) => {
                actix_web::HttpResponse::InternalServerError().json(AuthErrorResponse {
                    error: format!("Database error: {}", msg),
                })
            }
            AuthError::TokenError(msg) => {
                actix_web::HttpResponse::InternalServerError().json(AuthErrorResponse {
                    error: format!("Token error: {}", msg),
                })
            }
            AuthError::InvalidToken => {
                actix_web::HttpResponse::Unauthorized().json(AuthErrorResponse {
                    error: "Invalid or expired token".to_string(),
                })
            }
            AuthError::Unauthorized => {
                actix_web::HttpResponse::Forbidden().json(AuthErrorResponse {
                    error: "Unauthorized access".to_string(),
                })
            }
        }
    }
}
