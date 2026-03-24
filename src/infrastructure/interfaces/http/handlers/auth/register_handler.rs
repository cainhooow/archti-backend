use std::sync::Arc;

use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::{
        exceptions::{AppError, AppResult},
        usecases::user::user_usecases::CreateUserUseCase,
    },
    domain::builders::user_builder::UserBuilder,
    infrastructure::{
        http::State,
        interfaces::http::resources::{
            DataResponse,
            user_resource::{UserRequest, UserResource},
        },
        persistence::sea_orm_user_repository::SeaOrmUserRepository,
    },
};

#[handler]
async fn register_handler(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> AppResult<()> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| AppError::InternalServerError("Failed to obtain app state".to_string()))?;

    let repository = SeaOrmUserRepository::new(state.db.clone());

    match req.parse_body::<UserRequest>().await {
        Ok(validator) => {
            _ = validator.validate()?;

            let user_builder = UserBuilder::new();
            let user = user_builder
                .email(validator.email)
                .full_name(validator.full_name)
                .phone(validator.phone)
                .password_hash(validator.password_hash)
                .build();

            match CreateUserUseCase::new(repository).execute(&user).await {
                Ok(user) => {
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
