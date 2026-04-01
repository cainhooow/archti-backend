use std::sync::Arc;

use garde::Validate;
use salvo::prelude::*;
use serde::Serialize;

use crate::{
    application::usecases::user::password_reset_usecase::PasswordResetCommand,
    infrastructure::{
        http::{State, middlewares::auth_middleware::DEPOT_KEY_ID},
        interfaces::http::{
            exceptions::HttpError,
            resources::{DataResponse, auth_resources::PasswordResetRequest},
        },
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

    if let Ok(_) = depot.get::<String>(DEPOT_KEY_ID) {
        return Err(HttpError::Unauthorized(format!(
            "Account already connected. Un-login and try again later."
        )));
    }

    match req.parse_body::<PasswordResetRequest>().await {
        Ok(validator) => {
            validator.validate()?;
            let token = req
                .params()
                .get("token")
                .ok_or(HttpError::BadRequest("Token not found".to_string()))?
                .to_string();

            let is_changed = state
                .identity
                .reset_password(PasswordResetCommand {
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
