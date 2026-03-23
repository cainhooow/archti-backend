use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation, errors::Error};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub exp: i64,
    pub typ: String,
}

#[derive(Default, Debug)]
pub struct JwtAuthService {
    secret: String,
}

pub const ACCESS_TOKEN_NAME: &'static str = "accesstkn";
pub const REFRESH_TOKEN_NAME: &'static str = "refreshtkn";

#[derive(Serialize)]
pub struct Tokens {
    #[serde(rename(serialize = "accessToken"))]
    pub access_token: String,
    #[serde(rename(serialize = "refreshToken"))]
    pub refresh_token: String,
}

pub struct AuthorizationHeader<'l> {
    pub token_type: &'l str,
    pub token: &'l str,
}

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

    pub fn generate(&self, user_id: String) -> Result<Tokens, Error> {
        let access_token = self.generate_access_token(user_id.clone())?;
        let refresh_token = self.generate_refresh_token(user_id.clone())?;

        Ok(Tokens {
            access_token,
            refresh_token,
        })
    }

    pub fn generate_access_token(&self, user_id: String) -> Result<String, Error> {
        let claims = JwtClaims {
            sub: user_id,
            exp: (OffsetDateTime::now_utc() + Duration::minutes(15)).unix_timestamp(),
            typ: String::from(ACCESS_TOKEN_NAME),
        };

        let token = self.generate_token(claims)?;
        Ok(token)
    }

    pub fn generate_refresh_token(&self, user_id: String) -> Result<String, Error> {
        let claims = JwtClaims {
            sub: user_id,
            exp: (OffsetDateTime::now_utc() + Duration::days(7)).unix_timestamp(),
            typ: String::from(REFRESH_TOKEN_NAME),
        };

        let token = self.generate_token(claims)?;
        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<JwtClaims, String> {
        jsonwebtoken::decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(|e| e.to_string())
    }

    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<String, String> {
        let claims = self.validate_token(refresh_token)?;

        if claims.typ != "refreshtkn" {
            return Err("Invalid token type".into());
        }

        self.generate_access_token(claims.sub)
            .map_err(|e| e.to_string())
    }

    pub fn validate_from_authorization<'r>(
        &'r self,
        token: &'r str,
    ) -> Result<AuthorizationHeader<'r>, ()> {
        let parts: Vec<&str> = token.split(" ").collect();

        let token_type = parts[0];
        if !token_type.eq("Bearer") || token_type.is_empty() {
            return Err(());
        }

        let token = parts[1];

        Ok(AuthorizationHeader { token_type, token })
    }
}
