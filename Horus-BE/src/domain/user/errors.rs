use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use tracing::{debug, error};

/// Result type for user operations
pub type UserResult<T> = Result<T, UserError>;

/// Errors that can occur during user operations
#[derive(Error, Debug)]
pub enum UserError {
    /// Error when user already exists
    #[error("User already exists with that username or email")]
    UserAlreadyExists,

    /// Error when user doesn't exist
    #[error("User with ID {0} not found")]
    UserNotFound(i32),

    /// Error when user validation fails
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Error with password handling
    #[error("Password error: {0}")]
    PasswordError(String),

    /// Error with authentication
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    /// Error with authorization
    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    /// Error with user data
    #[error("User data error: {0}")]
    UserDataError(String),

    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Cache error
    #[error("Cache error: {0}")]
    CacheError(String),

    /// Internal error
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::UserAlreadyExists => {
                debug!(error = self.to_string(), "User already exists");
                HttpResponse::Conflict().json(self.to_string())
            }
            Self::UserNotFound(id) => {
                debug!(error = id, "User not found");
                HttpResponse::NotFound().json(self.to_string())
            }
            Self::ValidationError(msg) => {
                debug!(error = msg, "Validation error");
                HttpResponse::BadRequest().json(self.to_string())
            }
            Self::PasswordError(msg) => {
                debug!(error = msg, "Password error");
                HttpResponse::BadRequest().json(self.to_string())
            }
            Self::AuthenticationError(msg) => {
                debug!(error = msg, "Authentication error");
                HttpResponse::Unauthorized().json(self.to_string())
            }
            Self::AuthorizationError(msg) => {
                debug!(error = msg, "Authorization error");
                HttpResponse::Forbidden().json(self.to_string())
            }
            Self::UserDataError(msg) => {
                debug!(error = msg, "User data error");
                HttpResponse::BadRequest().json(self.to_string())
            }
            Self::DatabaseError(msg) => {
                debug!(error = msg, "Database error");
                HttpResponse::InternalServerError().json(self.to_string())
            }
            Self::CacheError(msg) => {
                debug!(error = msg, "Cache error");
                HttpResponse::InternalServerError().json(self.to_string())
            }
            Self::InternalError(msg) => {
                debug!(error = msg, "Internal error");
                HttpResponse::InternalServerError().json(self.to_string())
            }
        }
    }
}
