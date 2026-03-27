use std::sync::Arc;

use crate::application::exceptions::AppResult;

pub struct PasswordResetTokenOutput {
    pub token: String,
    pub expires_at: i64,
}

pub trait PasswordResetTokenService: Send + Sync {
    fn generate_reset_token(&self, user_id: &str) -> AppResult<PasswordResetTokenOutput>;
    fn verify_token(&self, token: &str) -> AppResult<String>;
}

impl PasswordResetTokenService for Arc<dyn PasswordResetTokenService> {
    fn generate_reset_token(&self, user_id: &str) -> AppResult<PasswordResetTokenOutput> {
        (**self).generate_reset_token(user_id)
    }

    fn verify_token(&self, token: &str) -> AppResult<String> {
        (**self).verify_token(token)
    }
}