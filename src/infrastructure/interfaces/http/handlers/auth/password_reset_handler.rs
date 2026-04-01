use std::sync::Arc;

use garde::Validate;
use salvo::prelude::*;
use serde::Serialize;

use crate::{
    application::usecases::user::password_reset_usecase::{
        PasswordResetCommand, PasswordResetUseCase,
    },
    infrastructure::{
        http::{State, middlewares::auth_middleware::DEPOT_KEY_ID},
        interfaces::http::{
            exceptions::HttpError,
            resources::{DataResponse, auth_resources::PasswordResetRequest},
        },
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
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| HttpError::InternalServerError(format!("Failed to obtain app state")))?;

    let repository = SeaOrmUserRepository::new(state.db.clone());
    let token_service = state.reset_token_service.clone();
    let hasher = state.hasher.clone();
    let sender = state.sender.clone();

    if let Ok(_) = depot.get::<String>(DEPOT_KEY_ID) {
        return Err(HttpError::Unauthorized(format!(
            "Account already connected. Un-login and try again later."
        )));
    }

    match req.parse_body::<PasswordResetRequest>().await {
        Ok(validator) => {
            _ = validator
                .validate()
                .map_err(|e| HttpError::BadRequest(e.to_string()))?;
            let token = req
                .params()
                .get("token")
                .ok_or(HttpError::BadRequest("Token not found".to_string()))?
                .to_string();

            let is_changed = PasswordResetUseCase::new(repository, token_service, hasher, sender)
                .execute(PasswordResetCommand {
                    token,
                    password: validator.password,
                })
                .await?;

            if !is_changed {
                return Err(HttpError::BadRequest(
                    "Invalid or expired token".to_string(),
                ));
            }

            res.render(DataResponse::success(PasswordResetResponse {
                message: "Password reset successfull".to_string(),
            }));
            res.status_code(StatusCode::OK);
        }
        Err(err) => {
            return Err(HttpError::BadRequest(err.to_string()));
        }
    }

    Ok(())
}
