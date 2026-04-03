use garde::Validate;
use salvo::prelude::*;
use std::sync::Arc;

use crate::{
    application::usecases::user::login_user_usecase::LoginUserCommand,
    infrastructure::{
        http::HttpState,
        interfaces::http::{
            exceptions::HttpError,
            resources::{
                DataResponse,
                auth_resources::{AuthRequest, AuthResource},
            },
        },
    },
};

#[handler]
pub async fn login_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<Arc<HttpState>>()
        .map_err(|_| HttpError::InternalServerError("Failed to obtain app state".to_string()))?;

    match req.parse_body::<AuthRequest>().await {
        Ok(validator) => {
            validator.validate()?;

            let command = LoginUserCommand {
                email: validator.email,
                password: validator.password,
            };

            let login_response = state.app.identity.login(command).await?;

            state.app.cookie_service.generate_sessions(
                &login_response.access_token,
                &login_response.refresh_token,
                res,
            );

            res.render(DataResponse::success(AuthResource::from(login_response)));
            res.status_code(StatusCode::OK);
            Ok(())
        }
        Err(err) => Err(HttpError::BadRequest(err.to_string())),
    }
}
