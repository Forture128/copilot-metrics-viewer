use actix_web::{HttpResponse, ResponseError};
use deadpool::managed::PoolError;
use diesel::result::Error as DieselError;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("GitHub API error: {0}")]
    GitHubError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Redis error: {0}")]
    RedisError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Server error: {0}")]
    ServerError(String),
}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl<T: std::fmt::Display> From<PoolError<T>> for AppError {
    fn from(err: PoolError<T>) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            status: "ERROR".to_string(),
            message: self.to_string(),
            error_code: Some(self.error_code()),
            details: self.error_details(),
        };

        match self {
            AppError::ValidationError(_) => HttpResponse::BadRequest().json(error_response),
            AppError::NotFound(_) => HttpResponse::NotFound().json(error_response),
            AppError::GitHubError(_) => HttpResponse::BadGateway().json(error_response),
            AppError::ServerError(_) => HttpResponse::InternalServerError().json(error_response),
            _ => HttpResponse::InternalServerError().json(error_response),
        }
    }
}

impl AppError {
    fn error_code(&self) -> String {
        match self {
            AppError::GitHubError(_) => "GITHUB_ERROR",
            AppError::DatabaseError(_) => "DB_ERROR",
            AppError::RedisError(_) => "REDIS_ERROR",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::InternalError(_) => "INTERNAL_ERROR",
            AppError::ServerError(_) => "SERVER_ERROR",
        }
        .to_string()
    }

    fn error_details(&self) -> Option<String> {
        match self {
            AppError::GitHubError(msg) => Some(format!("GitHub API error details: {}", msg)),
            AppError::DatabaseError(msg) => Some(format!("Database error details: {}", msg)),
            AppError::RedisError(msg) => Some(format!("Redis error details: {}", msg)),
            AppError::ServerError(msg) => Some(format!("Server error details: {}", msg)),
            _ => None,
        }
    }
}
