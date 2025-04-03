use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use tracing::error;
/// Result type for team operations
pub type TeamResult<T> = Result<T, TeamError>;

/// Errors that can occur during team operations
#[derive(Error, Debug)]
pub enum TeamError {
    /// Error when team already exists
    #[error("Team already exists with that name")]
    TeamAlreadyExists,

    /// Error when team doesn't exist
    #[error("Team with ID {0} not found")]
    TeamNotFound(i32),

    /// Error when user is already a member of the team
    #[error("User {0} is already a member of team {1}")]
    UserAlreadyInTeam(i32, i32),

    /// Error when user is not a member of the team
    #[error("User {0} is not a member of team {1}")]
    UserNotInTeam(i32, i32),

    /// Error when validation fails
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Error when a department doesn't exist
    #[error("Department with ID {0} not found")]
    DepartmentNotFound(i32),

    /// Error when a user doesn't exist
    #[error("User with ID {0} not found")]
    UserNotFound(i32),

    /// Error with repository association
    #[error("Repository error: {0}")]
    RepositoryError(String),

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

impl ResponseError for TeamError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::TeamAlreadyExists => HttpResponse::Conflict().json(self.to_string()),
            Self::TeamNotFound(error) => {
                error!(error = error, "Team not found");
                HttpResponse::NotFound().json(self.to_string())
            }
            Self::UserAlreadyInTeam(user_id, team_id) => {
                error!(user_id = user_id, team_id = team_id, "User already in team");
                HttpResponse::Conflict().json(self.to_string())
            }
            Self::UserNotInTeam(user_id, team_id) => {
                error!(user_id = user_id, team_id = team_id, "User not in team");
                HttpResponse::NotFound().json(self.to_string())
            }
            Self::ValidationError(error) => {
                error!(error = error, "Validation error in team service");
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
            Self::RepositoryError(error) => {
                error!(error = error, "Repository error");
                HttpResponse::BadRequest().json(self.to_string())
            }
            Self::DatabaseError(error) => {
                error!(error = error, "Database error in team service");
                HttpResponse::InternalServerError().json(self.to_string())
            }
            Self::CacheError(error) => {
                error!(error = error, "Cache error in team service");
                HttpResponse::InternalServerError().json(self.to_string())
            }
            Self::InternalError(error) => {
                error!(error = error, "Internal error in team service");
                HttpResponse::InternalServerError().json(self.to_string())
            }
        }
    }
}
