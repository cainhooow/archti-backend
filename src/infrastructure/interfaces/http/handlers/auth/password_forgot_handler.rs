use std::{env, sync::Arc};

use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::{
        exceptions::{AppError, AppResult},
        usecases::user::password_forgot_usecase::{
            RequestPasswordResetCommand, RequestPasswordResetUseCase,
        },
    },
    infrastructure::{
        http::State, interfaces::http::resources::auth_resource::PasswordForgotRequest,
        persistence::sea_orm_user_repository::SeaOrmUserRepository,
    },
};

#[handler]
pub async fn forgot_password_handler(
    req: &mut Request,
    depot: &Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| AppError::Unexpected(format!("Failed to obtain app state")))?;

    let repository = SeaOrmUserRepository::new(state.db.clone());
    let token_service = state.reset_token_service.clone();
    let sender = state.sender.clone();
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL is not defined in .env");

    match req.parse_body::<PasswordForgotRequest>().await {
        Ok(validator) => {
            _ = validator.validate()?;
            let email = validator.email;

            RequestPasswordResetUseCase::new(repository, token_service, sender, frontend_url)
                .execute(RequestPasswordResetCommand { email: email })
                .await?;

            Ok(())
        }
        Err(err) => {
            return Err(AppError::Bad(err.to_string()));
        }
    }
}
