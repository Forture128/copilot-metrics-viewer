use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use tracing::{debug, error, warn};
/// Result type for department operations
pub type DepartmentResult<T> = Result<T, DepartmentError>;

/// Errors that can occur during department operations
#[derive(Error, Debug)]
pub enum DepartmentError {
    /// Error when department already exists
    #[error("Department already exists with that name")]
    DepartmentAlreadyExists(String),

    /// Error when department doesn't exist
    #[error("Department with ID {0} not found")]
    DepartmentNotFound(i32),

    /// Error when validation fails
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Error with department data
    #[error("Department data error: {0}")]
    DepartmentDataError(String),

    /// Error when a user doesn't exist
    #[error("User with ID {0} not found")]
    UserNotFound(i32),

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

impl ResponseError for DepartmentError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::InternalError(error) => {
                error!(error = error, "Internal error in department service");
                HttpResponse::InternalServerError().json(self.to_string())
            }
            Self::DepartmentAlreadyExists(error) => {
                error!(error = error, "Department already exists");
                HttpResponse::Conflict().json(self.to_string())
            }
            Self::DepartmentNotFound(error) => {
                error!(error = error, "Department not found");
                HttpResponse::NotFound().json(self.to_string())
            }
            Self::ValidationError(error) => {
                error!(error = error, "Validation error in department service");
                HttpResponse::BadRequest().json(self.to_string())
            }
            Self::DepartmentDataError(error) => {
                error!(error = error, "Department data error");
                HttpResponse::BadRequest().json(self.to_string())
            }
            Self::UserNotFound(error) => {
                error!(error = error, "User not found");
                HttpResponse::NotFound().json(self.to_string())
            }
            Self::DatabaseError(error) => {
                error!(error = error, "Database error in department service");
                HttpResponse::InternalServerError().json(self.to_string())
            }
            Self::CacheError(error) => {
                error!(error = error, "Cache error in department service");
                HttpResponse::InternalServerError().json(self.to_string())
            }
        }
    }
}
