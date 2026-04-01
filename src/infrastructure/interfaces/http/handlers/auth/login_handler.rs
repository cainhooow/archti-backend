use garde::Validate;
use salvo::prelude::*;
use std::sync::Arc;

use crate::{
    application::usecases::user::login_user_usecase::LoginUserCommand,
    infrastructure::{
        http::State,
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
        .obtain::<Arc<State>>()
        .map_err(|_| HttpError::InternalServerError("Failed to obtain app state".to_string()))?;

    match req.parse_body::<AuthRequest>().await {
        Ok(validator) => {
            validator.validate()?;

            let command = LoginUserCommand {
                email: validator.email,
                password: validator.password,
            };

            let login_response = state.identity.login(command).await?;

            _ = state.cookie_service.generate_sessions(
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
