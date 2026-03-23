use std::num::{IntErrorKind, ParseIntError};

use crate::{
    application::exceptions::AppError, infrastructure::interfaces::http::resources::DataResponse,
};
use argon2::password_hash::Error as ArgonError;
use jsonwebtoken::errors::{Error as JWTError, ErrorKind as JWTErrorKind};

use salvo::prelude::*;

#[async_trait::async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, _res: &mut Response) {
        let (status, message) = match &self {
            AppError::Domain(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Repository(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Unexpected(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Bad(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::InvalidCredentials(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::EncryptionError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
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
            _ => AppError::EncryptionError(format!(
                "Argon2Error: Failed to hash password. Err: {}",
                value.to_string()
            )),
        }
    }
}

impl From<JWTError> for AppError {
    fn from(value: JWTError) -> Self {
        println!("JWTError: {}", value);
        let error_kind = value.kind();
        match error_kind {
            JWTErrorKind::InvalidToken => {
                AppError::Unauthorized(format!("Invalid JWToken: {}", value.to_string()))
            }
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
        AppError::Domain(message.to_string())
    }
}