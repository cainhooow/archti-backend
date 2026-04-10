use std::sync::Arc;

use chrono::NaiveDateTime;

use crate::application::exceptions::AppResult;

pub struct PasswordResetTokenOutput {
    pub token: String,
    pub expires_at: i64,
}

pub trait PasswordResetTokenService: Send + Sync {
    fn generate_reset_token(&self, user_id: &i64) -> AppResult<PasswordResetTokenOutput>;
    fn verify_token(&self, token: &str) -> AppResult<i64>;
    fn validate_token(&self, token: &str, last_pass_change: NaiveDateTime) -> AppResult<i64>;
}

impl PasswordResetTokenService for Arc<dyn PasswordResetTokenService> {
    fn generate_reset_token(&self, user_id: &i64) -> AppResult<PasswordResetTokenOutput> {
        (**self).generate_reset_token(user_id)
    }

    fn verify_token(&self, token: &str) -> AppResult<i64> {
        (**self).verify_token(token)
    }

    fn validate_token(&self, token: &str, last_pass_change: NaiveDateTime) -> AppResult<i64> {
        (**self).validate_token(token, last_pass_change)
    }
}
