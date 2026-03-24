use std::{fmt::Debug, sync::Arc};

use crate::application::exceptions::AppResult;

pub struct TokenOutput {
    pub token: String,
    pub expires_at: i64,
}

pub trait TokenService: Send + Sync + Debug {
    fn generate_access_token(&self, user_id: &str) -> AppResult<TokenOutput>;
    fn generate_refresh_token(&self, user_id: &str) -> AppResult<TokenOutput>;
    fn verify_token(&self, token: &str) -> AppResult<String>;
}

impl TokenService for Arc<dyn TokenService> {
    fn generate_access_token(&self, user_id: &str) -> AppResult<TokenOutput> {
        (**self).generate_access_token(user_id)
    }

    fn generate_refresh_token(&self, user_id: &str) -> AppResult<TokenOutput> {
        (**self).generate_refresh_token(user_id)
    }

    fn verify_token(&self, token: &str) -> AppResult<String> {
        (**self).verify_token(token)
    }
}