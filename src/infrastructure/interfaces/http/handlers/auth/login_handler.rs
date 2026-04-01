use garde::Validate;
use salvo::prelude::*;
use std::sync::Arc;

use crate::{
    application::usecases::user::login_user_usecase::{LoginUserCommand, LoginUserUseCase},
    infrastructure::{
        http::State,
        interfaces::http::{
            exceptions::HttpError,
            resources::{
                DataResponse,
                auth_resources::{AuthRequest, AuthResource},
            },
        },
        persistence::sea_orm_user_repository::SeaOrmUserRepository,
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

    let repository = SeaOrmUserRepository::new(state.db.clone());
    let token_service = state.auth_service.clone();
    let hasher = state.hasher.clone();

    match req.parse_body::<AuthRequest>().await {
        Ok(validator) => {
            _ = validator
                .validate()
                .map_err(|err| HttpError::BadRequest(err.to_string()));

            let command = LoginUserCommand {
                email: validator.email,
                password: validator.password,
            };

            match LoginUserUseCase::new(repository, token_service, hasher)
                .execute(command)
                .await
            {
                Ok(login_response) => {
                    _ = state.cookie_service.generate_sessions(
                        &login_response.access_token,
                        &login_response.refresh_token,
                        res,
                    );

                    res.render(DataResponse::success(AuthResource::from(login_response)));
                    res.status_code(StatusCode::OK);
                    Ok(())
                }
                Err(err) => Err(HttpError::InternalServerError(err.to_string())),
            }
        }
        Err(err) => Err(HttpError::BadRequest(err.to_string())),
    }
}
