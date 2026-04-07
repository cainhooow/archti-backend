use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Validation,
    errors::{Error, ErrorKind},
};
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
    pub iat: i64,
}

#[derive(Debug, Default)]
pub struct JwtAuthService {
    secret: String,
}

pub const ACCESS_TOKEN_NAME: &str = "accesstkn";
pub const REFRESH_TOKEN_NAME: &str = "refreshtkn";

impl JwtAuthService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    fn map_token_generation_error(err: Error) -> AppError {
        AppError::External(format!("Failed to generate jwt token: {err}"))
    }

    fn map_token_decode_error(err: Error) -> AppError {
        match err.kind() {
            ErrorKind::InvalidToken | ErrorKind::ExpiredSignature => AppError::AuthenticationFailed,
            _ => AppError::Unexpected(format!("Failed to decode jwt token: {err}")),
        }
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
        let now = OffsetDateTime::now_utc().unix_timestamp();

        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp,
            typ: ACCESS_TOKEN_NAME.to_string(),
            iat: now,
        };

        let token = self
            .generate_token(claims)
            .map_err(Self::map_token_generation_error)?;

        Ok(TokenOutput {
            token,
            expires_at: exp,
        })
    }

    fn generate_refresh_token(&self, user_id: &str) -> AppResult<TokenOutput> {
        let exp = (OffsetDateTime::now_utc() + Duration::days(7)).unix_timestamp();
        let now = OffsetDateTime::now_utc().unix_timestamp();

        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp,
            typ: REFRESH_TOKEN_NAME.to_string(),
            iat: now,
        };

        let token = self
            .generate_token(claims)
            .map_err(Self::map_token_generation_error)?;

        Ok(TokenOutput {
            token,
            expires_at: exp,
        })
    }

    fn verify_token(&self, token: &str) -> AppResult<String> {
        let claims = self
            .decode_token(token)
            .map_err(Self::map_token_decode_error)?;

        if claims.typ != ACCESS_TOKEN_NAME {
            return Err(AppError::AuthenticationFailed);
        }

        Ok(claims.sub)
    }

    fn get_refresh_sub(&self, token: &str) -> AppResult<String> {
        let claims = self
            .decode_token(token)
            .map_err(Self::map_token_decode_error)?;

        if claims.typ != REFRESH_TOKEN_NAME {
            return Err(AppError::AuthenticationFailed);
        }

        Ok(claims.sub)
    }

    fn renew_token(&self, token: &str) -> AppResult<TokenOutput> {
        let claims = self
            .decode_token(token)
            .map_err(Self::map_token_decode_error)?;

        if claims.typ != REFRESH_TOKEN_NAME {
            return Err(AppError::AuthenticationFailed);
        }

        let exp = (OffsetDateTime::now_utc() + Duration::minutes(15)).unix_timestamp();
        let now = OffsetDateTime::now_utc().unix_timestamp();

        let new_claims = JwtClaims {
            sub: claims.sub,
            exp,
            typ: ACCESS_TOKEN_NAME.to_string(),
            iat: now,
        };

        let new_token = self
            .generate_token(new_claims)
            .map_err(Self::map_token_generation_error)?;

        Ok(TokenOutput {
            token: new_token,
            expires_at: exp,
        })
    }
}
