use std::sync::Arc;

use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::usecases::user::password_change_usecase::ChangePasswordCommand,
    infrastructure::{
        http::{HttpState, middlewares::auth_middleware::DEPOT_KEY_ID},
        http::{
            exceptions::HttpError,
            resources::{
                DataResponse, me_resources::ChangePasswordRequest,
                message_resource::MessageResource,
            },
        },
    },
};

#[handler]
pub async fn change_password_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<Arc<HttpState>>()
        .map_err(|_| HttpError::InternalServerError("Failed to obtain app state".to_string()))?;

    let user_id = depot
        .get::<String>(DEPOT_KEY_ID)
        .map_err(|_| HttpError::InternalServerError("Failed to get user depot key".to_string()))?
        .to_owned();

    match req.parse_body::<ChangePasswordRequest>().await {
        Ok(validator) => {
            validator.validate()?;

            let is_changed = state
                .app
                .identity
                .change_password(ChangePasswordCommand {
                    old_password: validator.old_password,
                    new_password: validator.new_password,
                    user_id,
                })
                .await?;

            if is_changed {
                res.status_code(StatusCode::OK);
                res.render(DataResponse::success(MessageResource {
                    message: "Password changed successfully.".to_string(),
                }));
            } else {
                res.status_code(StatusCode::FORBIDDEN);
                res.render(DataResponse::error(MessageResource {
                    message: "Failed to change password".to_string(),
                }));
            }
        }
        Err(err) => return Err(HttpError::BadRequest(err.to_string())),
    }

    Ok(())
}
