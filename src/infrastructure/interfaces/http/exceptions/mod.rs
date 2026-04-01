use std::num::{IntErrorKind, ParseIntError};
use thiserror::Error;

use crate::application::exceptions::AppError;
use crate::infrastructure::interfaces::http::resources::DataResponse;
use argon2::password_hash::Error as ArgonError;
use jsonwebtoken::errors::{Error as JWTError, ErrorKind as JWTErrorKind};

use salvo::prelude::*;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    InternalServerError(String),
}

impl From<AppError> for HttpError {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Validation(msg) => HttpError::BadRequest(msg),
            AppError::RuleViolation(msg) => HttpError::BadRequest(msg),
            AppError::NotFound(msg) => HttpError::NotFound(msg),
            AppError::Conflict(msg) => HttpError::Conflict(msg),
            AppError::AuthenticationFailed => {
                HttpError::Unauthorized("Authentication failed".to_string())
            }
            AppError::PermissionDenied => HttpError::Forbidden("Permission denied".to_string()),
            AppError::External(msg) => HttpError::InternalServerError(msg),
            AppError::Unexpected(msg) => HttpError::InternalServerError(msg),
        }
    }
}

#[async_trait::async_trait]
impl Writer for HttpError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, _res: &mut Response) {
        let (status, message) = match self {
            HttpError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            HttpError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            HttpError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            HttpError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            HttpError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            HttpError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        _res.status_code(status);
        _res.render(DataResponse::error(message));
    }
}

impl From<ParseIntError> for AppError {
    fn from(value: ParseIntError) -> Self {
        println!("ParseIntError: {}", value);
        let error_kind = value.kind();
        match error_kind {
            IntErrorKind::InvalidDigit => {
                AppError::Unexpected(format!("Provided value is invalid int(i32, i64)"))
            }
            _ => AppError::Unexpected(format!("ParseIntError")),
        }
    }
}

impl From<ArgonError> for AppError {
    fn from(value: ArgonError) -> Self {
        println!("Argon2Error: {}", value);
        match value {
            _ => AppError::Unexpected(format!("Argon2Error: {}", value.to_string())),
        }
    }
}

impl From<JWTError> for AppError {
    fn from(value: JWTError) -> Self {
        println!("JWTError: {}", value);
        let error_kind = value.kind();
        match error_kind {
            JWTErrorKind::InvalidToken => AppError::AuthenticationFailed,
            JWTErrorKind::Json(msg) => {
                AppError::Unexpected(format!("Invalid JWToken: {}", msg.to_string()))
            }
            _ => AppError::Unexpected(format!("JWTError: {}", value.to_string())),
        }
    }
}

impl From<garde::Report> for AppError {
    fn from(value: garde::Report) -> Self {
        let message = value.to_string();
        AppError::Validation(message)
    }
}
