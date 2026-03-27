use garde::Validate;
use salvo::prelude::*;

use crate::{application::exceptions::{AppError, AppResult}, infrastructure::interfaces::http::resources::auth_resource::PasswordResetRequest};

#[handler]
pub async fn forgot_password_handler(req: &mut Request, depot: &Depot, res: &mut Response) -> AppResult<()> {
    match req.parse_body::<PasswordResetRequest>().await {
        Ok(validator) => {
            _ = validator.validate();

        }
        Err(err) => {
            return Err(AppError::Bad(err.to_string()));
        }
    }
    Ok(())
}