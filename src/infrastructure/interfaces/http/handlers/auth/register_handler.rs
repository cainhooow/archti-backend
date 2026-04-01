use std::sync::Arc;

use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::usecases::user::create_user_usecase::{CreateUserCommand, CreateUserUseCase},
    infrastructure::{
        http::State,
        interfaces::http::{
            exceptions::HttpError,
            resources::{
                DataResponse,
                user_resources::{UserRequest, UserResource},
            },
        },
        persistence::sea_orm_user_repository::SeaOrmUserRepository,
    },
};

#[handler]
pub async fn register_handler(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| HttpError::InternalServerError("Failed to obtain app state".to_string()))?;

    let repository = SeaOrmUserRepository::new(state.db.clone());
    let hasher = state.hasher.clone();

    match req.parse_body::<UserRequest>().await {
        Ok(validator) => {
            validator.validate()?;

            let user = CreateUserUseCase::new(repository, hasher, state.sender.clone())
                .execute(CreateUserCommand {
                    email: validator.email,
                    password: validator.password,
                    full_name: validator.full_name,
                    phone: validator.phone,
                })
                .await?;

            res.render(DataResponse::success(UserResource::from(user)));
            Ok(())
        }
        Err(err) => {
            return Err(HttpError::BadRequest(err.to_string()));
        }
    }
}
