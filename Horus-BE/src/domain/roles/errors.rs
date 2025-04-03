use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use tracing::{debug, error, warn};

/// Result type for role operations
pub type RoleResult<T> = Result<T, RoleError>;

/// Errors that can occur during role operations
#[derive(Error, Debug)]
pub enum RoleError {
    /// Error when role already exists
    #[error("Role already exists with that name")]
    RoleAlreadyExists,

    /// Error when role doesn't exist
    #[error("Role with ID {0} not found")]
    RoleNotFound(i32),

    /// Error when validation fails
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Error with role data
    #[error("Role data error: {0}")]
    RoleDataError(String),

    /// Error when a department doesn't exist
    #[error("Department with ID {0} not found")]
    DepartmentNotFound(i32),

    /// Error when a user doesn't exist
    #[error("User with ID {0} not found")]
    UserNotFound(i32),

    /// Error when department role already assigned
    #[error("User {0} already has role {1} in department {2}")]
    DepartmentRoleAlreadyAssigned(i32, i32, i32),

    /// Error when department role not assigned
    #[error("User {0} does not have role {1} in department {2}")]
    DepartmentRoleNotAssigned(i32, i32, i32),

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

impl ResponseError for RoleError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::RoleAlreadyExists => HttpResponse::Conflict().json(self.to_string()),
            Self::RoleNotFound(error) => {
                error!(error = error, "Role not found");
                HttpResponse::NotFound().json(self.to_string())
            }
            Self::ValidationError(error) => {
                error!(error = error, "Validation error in role service");
                HttpResponse::BadRequest().json(self.to_string())
            }
            Self::RoleDataError(error) => {
                error!(error = error, "Role data error");
                HttpResponse::BadRequest().json(self.to_string())
            }
            Self::DepartmentNotFound(error) => {
                error!(error = error, "Department not found");
                HttpResponse::NotFound().json(self.to_string())
            }
            Self::UserNotFound(error) => {
                error!(error = error, "User not found");
                HttpResponse::NotFound().json(self.to_string())
            }
            Self::DepartmentRoleAlreadyAssigned(id, role_id, department_id) => {
                error!(
                    "Department role already assigned: id={}, role_id={}, department_id={}",
                    id, role_id, department_id
                );
                HttpResponse::Conflict().json(self.to_string())
            }
            Self::DepartmentRoleNotAssigned(id, role_id, department_id) => {
                error!(
                    "Department role not assigned: id={}, role_id={}, department_id={}",
                    id, role_id, department_id
                );
                HttpResponse::NotFound().json(self.to_string())
            }
            Self::DatabaseError(error) => {
                error!(error = error, "Database error in role service");
                HttpResponse::InternalServerError().json(self.to_string())
            }
            Self::CacheError(error) => {
                error!(error = error, "Cache error in role service");
                HttpResponse::InternalServerError().json(self.to_string())
            }
            Self::InternalError(error) => {
                error!(error = error, "Internal error in role service");
                HttpResponse::InternalServerError().json(self.to_string())
            }
        }
    }
}
