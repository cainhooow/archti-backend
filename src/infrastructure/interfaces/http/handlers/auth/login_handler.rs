use garde::Validate;
use salvo::prelude::*;
use std::sync::Arc;

use crate::{
    application::{
        exceptions::{AppError, AppResult},
        usecases::user::login_user_usecase::{LoginUserCommand, LoginUserUseCase},
    },
    infrastructure::{
        http::State,
        interfaces::http::resources::{
            DataResponse,
            auth_resource::{AuthRequest, AuthResource},
        },
        persistence::sea_orm_user_repository::SeaOrmUserRepository,
    },
};

#[handler]
pub async fn login_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| AppError::InternalServerError("Failed to obtain app state".to_string()))?;

    let repository = SeaOrmUserRepository::new(state.db.clone());
    let token_service = state.auth_service.clone();
    let hasher = state.hasher.clone();

    match req.parse_body::<AuthRequest>().await {
        Ok(validator) => {
            _ = validator.validate()?;

            let command = LoginUserCommand {
                email: validator.email,
                password: validator.password,
            };

            match LoginUserUseCase::new(repository, token_service, hasher)
                .execute(command)
                .await
            {
                Ok(data) => {
                    res.render(DataResponse::success(AuthResource::from(data)));
                    res.status_code(StatusCode::OK);
                    Ok(())
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Err(err) => {
            return Err(AppError::Bad(err.to_string()));
        }
    }
}
