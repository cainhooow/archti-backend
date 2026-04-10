use chrono::NaiveDateTime;
use jsonwebtoken::{
    DecodingKey, EncodingKey, Validation,
    errors::{Error, ErrorKind},
};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::application::{
    exceptions::{AppError, AppResult},
    ports::password_reset_token_service::{PasswordResetTokenOutput, PasswordResetTokenService},
};

pub const PASSWORD_RESET_TOKEN_TYPE: &str = "pswdrst";

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetClaims {
    sub: i64,
    exp: i64,
    iat: i64,
    typ: String,
}

#[derive(Debug, Default)]
pub struct JwtPasswordResetTokenService {
    secret: String,
}

impl JwtPasswordResetTokenService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    fn map_token_generation_error(err: Error) -> AppError {
        AppError::External(format!("Failed to generate password reset token: {err}"))
    }

    fn map_token_decode_error(err: Error) -> AppError {
        match err.kind() {
            ErrorKind::InvalidToken | ErrorKind::ExpiredSignature => AppError::AuthenticationFailed,
            _ => AppError::Unexpected(format!("Failed to decode password reset token: {err}")),
        }
    }

    pub fn generate(&self, claims: PasswordResetClaims) -> AppResult<String> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(Self::map_token_generation_error)
    }

    pub fn decode(&self, token: &str) -> AppResult<PasswordResetClaims> {
        jsonwebtoken::decode::<PasswordResetClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(Self::map_token_decode_error)
    }
}

impl PasswordResetTokenService for JwtPasswordResetTokenService {
    fn generate_reset_token(&self, user_id: &i64) -> AppResult<PasswordResetTokenOutput> {
        let exp = (OffsetDateTime::now_utc() + Duration::minutes(30)).unix_timestamp();
        let now = OffsetDateTime::now_utc().unix_timestamp();

        let claims = PasswordResetClaims {
            sub: *user_id,
            exp,
            iat: now,
            typ: PASSWORD_RESET_TOKEN_TYPE.to_string(),
        };

        let token = self.generate(claims)?;

        Ok(PasswordResetTokenOutput {
            token,
            expires_at: exp,
        })
    }

    fn verify_token(&self, token: &str) -> AppResult<i64> {
        let claims = self.decode(token)?;

        if claims.typ != PASSWORD_RESET_TOKEN_TYPE {
            return Err(AppError::AuthenticationFailed);
        }

        Ok(claims.sub)
    }

    fn validate_token(&self, token: &str, last_pass_change: NaiveDateTime) -> AppResult<i64> {
        let claims = self.decode(token)?;

        if claims.typ != PASSWORD_RESET_TOKEN_TYPE {
            return Err(AppError::AuthenticationFailed);
        }

        if claims.iat <= last_pass_change.and_utc().timestamp() {
            return Err(AppError::AuthenticationFailed);
        }

        Ok(claims.sub)
    }
}
