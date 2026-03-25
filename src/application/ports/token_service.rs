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
    fn get_refresh_sub(&self, token: &str) -> AppResult<String>;
    fn renew_token(&self, token: &str) -> AppResult<TokenOutput>;
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
    
    fn get_refresh_sub(&self, token: &str) -> AppResult<String> {
        (**self).get_refresh_sub(token)
    }
    
    fn renew_token(&self, token: &str) -> AppResult<TokenOutput> {
        (**self).renew_token(token)
    }
}