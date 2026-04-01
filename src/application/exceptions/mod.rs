use thiserror::Error;

use crate::domain::exceptions::RepositoryError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Business rule violation: {0}")]
    RuleViolation(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Permission denied")]
    PermissionDenied,

    #[error("External dependency error: {0}")]
    External(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<String> for AppError {
    fn from(value: String) -> Self {
        AppError::Unexpected(value)
    }
}

impl From<RepositoryError> for AppError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound => AppError::NotFound("Entity not found".into()),
            RepositoryError::Generic(e) => AppError::Unexpected(e),
        }
    }
}
