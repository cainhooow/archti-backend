use thiserror::Error;

use crate::domain::exceptions::{MailerError, RepositoryError};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Domain error: {0}")]
    Domain(String),
    #[error("Repository error: {0}")]
    Repository(String),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Http Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Http Forbidden: {0}")]
    Forbidden(String),
    #[error("Http Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("Http Bad Request: {0}")]
    Bad(String),
    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),
    #[error("Encryption error: {0}")]
    EncryptionError(String),
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
            RepositoryError::Generic(e) => AppError::Repository(e),
            RepositoryError::NotFound => AppError::Repository("Entity not found".into()),
        }
    }
}

impl From<MailerError> for AppError {
    fn from(value: MailerError) -> Self {
        match value {
            MailerError::Generic(e) => AppError::Unexpected(e),
        }
    }
}
