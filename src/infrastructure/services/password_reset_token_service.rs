use chrono::NaiveDateTime;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::application::{
    exceptions::{AppError, AppResult},
    ports::password_reset_token_service::{PasswordResetTokenOutput, PasswordResetTokenService},
};

pub const PASSWORD_RESET_TOKEN_TYPE: &str = "pswdrst";

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetClaims {
    sub: String,
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

    pub fn generate(&self, claims: PasswordResetClaims) -> AppResult<String> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|err| AppError::Unexpected(err.to_string()))
    }

    pub fn decode(&self, token: &str) -> AppResult<PasswordResetClaims> {
        jsonwebtoken::decode::<PasswordResetClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(|err| AppError::Unexpected(err.to_string()))
    }
}

impl PasswordResetTokenService for JwtPasswordResetTokenService {
    fn generate_reset_token(&self, user_id: &str) -> AppResult<PasswordResetTokenOutput> {
        let exp = (OffsetDateTime::now_utc() + Duration::minutes(30)).unix_timestamp();
        let now = OffsetDateTime::now_utc().unix_timestamp();

        let claims = PasswordResetClaims {
            sub: user_id.to_string(),
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

    fn verify_token(&self, token: &str) -> AppResult<String> {
        let claims = self.decode(token)?;

        if claims.typ != PASSWORD_RESET_TOKEN_TYPE {
            return Err(AppError::Domain("Invalid token type".to_string()));
        }

        Ok(claims.sub)
    }

    fn validate_token(&self, token: &str, last_pass_change: NaiveDateTime) -> AppResult<String> {
        let claims = self.decode(token)?;

        if claims.typ != PASSWORD_RESET_TOKEN_TYPE {
            return Err(AppError::Domain("Invalid token type".to_string()));
        }

        if claims.iat <= last_pass_change.and_utc().timestamp() {
            return Err(AppError::Domain(
                "Token already used or expired".to_string(),
            ));
        }

        Ok(claims.sub)
    }
}
