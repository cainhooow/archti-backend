use std::sync::Arc;

use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::{
        exceptions::{AppError, AppResult},
        usecases::user::create_user_usecase::{CreateUserCommand, CreateUserUseCase},
    },
    domain::events::DomainEvents,
    infrastructure::{
        http::State,
        interfaces::http::resources::{
            DataResponse,
            user_resource::{UserRequest, UserResource},
        },
        persistence::sea_orm_user_repository::SeaOrmUserRepository,
    },
};
use tracing::error;

#[handler]
pub async fn register_handler(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> AppResult<()> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| AppError::InternalServerError("Failed to obtain app state".to_string()))?;

    let repository = SeaOrmUserRepository::new(state.db.clone());
    let hasher = state.hasher.clone();

    match req.parse_body::<UserRequest>().await {
        Ok(validator) => {
            _ = validator.validate()?;

            match CreateUserUseCase::new(repository, hasher)
                .execute(CreateUserCommand {
                    email: validator.email,
                    password: validator.password,
                    full_name: validator.full_name,
                    phone: validator.phone,
                })
                .await
            {
                Ok(user) => {
                    if let Err(err) = state.sender.send(DomainEvents::UserRegistered {
                            email: user.email().to_string().clone(),
                            name: user.full_name().to_string().clone(),
                        }) {
                        error!(%err, "failed to queue welcome notification");
                    }

                    res.render(DataResponse::success(UserResource::from(user)));
                    return Ok(());
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Err(err) => {
            return Err(AppError::Bad(err.to_string()));
        }
    };
}
