use std::sync::Arc;

use garde::Validate;
use salvo::prelude::*;
use serde::Serialize;

use crate::{
    application::{
        exceptions::{AppError, AppResult},
        usecases::user::password_reset_usecase::{PasswordResetCommand, PasswordResetUseCase},
    },
    infrastructure::{
        http::State,
        interfaces::http::resources::{DataResponse, auth_resource::PasswordResetRequest},
        persistence::sea_orm_user_repository::SeaOrmUserRepository,
    },
};

#[derive(Serialize)]
pub struct PasswordResetResponse {
    message: String,
}

#[handler]
pub async fn password_reset_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| AppError::Unexpected(format!("Failed to obtain app state.")))?;

    let repository = SeaOrmUserRepository::new(state.db.clone());
    let token_service = state.reset_token_service.clone();
    let hasher = state.hasher.clone();
    let sender = state.sender.clone();

    match req.parse_body::<PasswordResetRequest>().await {
        Ok(validator) => {
            _ = validator.validate()?;
            let token = req
                .params()
                .get("token")
                .ok_or(AppError::Bad("Token not found".to_string()))?
                .to_string();

            let is_changed = PasswordResetUseCase::new(repository, token_service, hasher, sender)
                .execute(PasswordResetCommand {
                    token,
                    password: validator.password,
                })
                .await?;

            if !is_changed {
                return Err(AppError::Bad("Invalid or expired token".to_string()));
            }

            res.render(DataResponse::success(PasswordResetResponse {
                message: "Password reset successfull".to_string(),
            }));
            res.status_code(StatusCode::OK);
        }
        Err(err) => {
            return Err(AppError::Bad(err.to_string()));
        }
    }

    Ok(())
}
