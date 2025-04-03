use actix_web::{HttpResponse, ResponseError};
use anyhow;
use thiserror::Error;
use tracing::{debug, error, warn};

#[derive(Error, Debug)]
pub enum OrganizationError {
    #[error("Organization not found with id: {0}")]
    NotFound(i32),

    #[error("Organization config not found: org_id={org_id}, config_id={config_id}")]
    ConfigNotFound { org_id: i32, config_id: i32 },

    #[error("Invalid organization data: {0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Unauthorized access to organization: {0}")]
    Unauthorized(i32),

    #[error("Redis error: {0}")]
    RedisError(#[from] anyhow::Error),
}

impl ResponseError for OrganizationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::NotFound(id) => {
                debug!(org_id = id, "Organization not found");
                HttpResponse::NotFound().json(self.to_string())
            }
            Self::ConfigNotFound { org_id, config_id } => {
                debug!(
                    org_id = org_id,
                    config_id = config_id,
                    "Organization config not found"
                );
                HttpResponse::NotFound().json(self.to_string())
            }
            Self::ValidationError(msg) => {
                warn!(error = msg, "Organization validation error");
                HttpResponse::BadRequest().json(self.to_string())
            }
            Self::Unauthorized(id) => {
                warn!(org_id = id, "Unauthorized access to organization");
                HttpResponse::Unauthorized().json(self.to_string())
            }
            Self::RedisError(err) => {
                error!(error = %err, "Redis error in organization service");
                HttpResponse::InternalServerError().json(self.to_string())
            }
            Self::DatabaseError(err) => {
                error!(error = err, "Database error in organization service");
                HttpResponse::InternalServerError().json(self.to_string())
            }
            _ => {
                error!(error = %self, "Unexpected organization error");
                HttpResponse::InternalServerError().json(self.to_string())
            }
        }
    }
}

pub type OrganizationResult<T> = Result<T, OrganizationError>;
