use std::sync::Arc;

use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::usecases::user::password_forgot_usecase::RequestPasswordResetCommand,
    infrastructure::{
        http::{HttpState, middlewares::auth_middleware::DEPOT_KEY_ID},
        interfaces::http::{
            exceptions::HttpError, resources::auth_resources::PasswordForgotRequest,
        },
    },
};

#[handler]
pub async fn forgot_password_handler(
    req: &mut Request,
    depot: &Depot,
    _res: &mut Response,
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<Arc<HttpState>>()
        .map_err(|_| HttpError::InternalServerError("Failed to obtain app state".to_string()))?;

    if let Ok(_) = depot.get::<String>(DEPOT_KEY_ID) {
        return Err(HttpError::Unauthorized(
            "Account already connected. Un-login and try again later.".to_string(),
        ));
    }

    match req.parse_body::<PasswordForgotRequest>().await {
        Ok(validator) => {
            validator.validate()?;
            let email = validator.email;

            state
                .app
                .identity
                .request_password_reset(RequestPasswordResetCommand { email })
                .await?;

            Ok(())
        }
        Err(err) => Err(HttpError::BadRequest(err.to_string())),
    }
}
