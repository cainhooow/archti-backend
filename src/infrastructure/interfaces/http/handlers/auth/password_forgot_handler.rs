use std::{env, sync::Arc};

use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::usecases::user::password_forgot_usecase::{
        RequestPasswordResetCommand, RequestPasswordResetUseCase,
    },
    infrastructure::{
        http::{State, middlewares::auth_middleware::DEPOT_KEY_ID},
        interfaces::http::{
            exceptions::HttpError, resources::auth_resources::PasswordForgotRequest,
        },
        persistence::sea_orm_user_repository::SeaOrmUserRepository,
    },
};

#[handler]
pub async fn forgot_password_handler(
    req: &mut Request,
    depot: &Depot,
    _res: &mut Response,
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| HttpError::InternalServerError(format!("Failed to obtain app state")))?;

    let repository = SeaOrmUserRepository::new(state.db.clone());
    let token_service = state.reset_token_service.clone();
    let sender = state.sender.clone();
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL is not defined in .env");

    if let Ok(_) = depot.get::<String>(DEPOT_KEY_ID) {
        return Err(HttpError::Unauthorized(format!(
            "Account already connected. Un-login and try again later."
        )));
    }

    match req.parse_body::<PasswordForgotRequest>().await {
        Ok(validator) => {
            validator.validate()?;
            let email = validator.email;

            RequestPasswordResetUseCase::new(repository, token_service, sender, frontend_url)
                .execute(RequestPasswordResetCommand { email: email })
                .await?;

            Ok(())
        }
        Err(err) => Err(HttpError::BadRequest(err.to_string())),
    }
}
