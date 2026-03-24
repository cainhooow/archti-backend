use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation, errors::Error};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::application::{
    exceptions::{AppError, AppResult},
    ports::token_service::{TokenOutput, TokenService},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub exp: i64,
    pub typ: String,
}

#[derive(Debug, Default)]
pub struct JwtAuthService {
    secret: String,
}

pub const ACCESS_TOKEN_NAME: &'static str = "accesstkn";
pub const REFRESH_TOKEN_NAME: &'static str = "refreshtkn";

impl JwtAuthService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    fn generate_token(&self, claims: JwtClaims) -> Result<String, Error> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;

        Ok(token)
    }

    fn decode_token(&self, token: &str) -> Result<JwtClaims, Error> {
        jsonwebtoken::decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map(|data| data.claims)
    }
}

impl TokenService for JwtAuthService {
    fn generate_access_token(&self, user_id: &str) -> AppResult<TokenOutput> {
        let exp = (OffsetDateTime::now_utc() + Duration::minutes(15)).unix_timestamp();
        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp,
            typ: ACCESS_TOKEN_NAME.to_string(),
        };

        let token = self
            .generate_token(claims)
            .map_err(|err| AppError::Unexpected(err.to_string()))?;

        Ok(TokenOutput {
            token,
            expires_at: exp,
        })
    }

    fn generate_refresh_token(&self, user_id: &str) -> AppResult<TokenOutput> {
        let exp = (OffsetDateTime::now_utc() + Duration::days(7)).unix_timestamp();
        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp,
            typ: REFRESH_TOKEN_NAME.to_string(),
        };

        let token = self
            .generate_token(claims)
            .map_err(|err| AppError::Unexpected(err.to_string()))?;

        Ok(TokenOutput {
            token,
            expires_at: exp,
        })
    }

    fn verify_token(&self, token: &str) -> AppResult<String> {
        let claims = self
            .decode_token(token)
            .map_err(|err| AppError::Unexpected(err.to_string()))?;

        if claims.typ != ACCESS_TOKEN_NAME {
            return Err(AppError::Unexpected("Invalid token type".to_string()));
        }

        Ok(claims.sub)
    }
}
