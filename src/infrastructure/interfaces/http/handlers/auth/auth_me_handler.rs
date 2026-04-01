use std::sync::Arc;

use salvo::prelude::*;

use crate::{
    application::{
        exceptions::{AppError},
        queries::user::find_user_by_id::{FindUserById, FindUserByIdQuery},
    },
    infrastructure::{
        http::{State, middlewares::auth_middleware::DEPOT_KEY_ID},
        interfaces::http::{
            exceptions::HttpError,
            resources::{DataResponse, user_resources::UserResource},
        },
        persistence::sea_orm_user_repository::SeaOrmUserRepository,
    },
};

#[handler]
pub async fn auth_me_handler(
    _req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| AppError::Unexpected(format!("Failed to obtain app state")))?;

    let repository = SeaOrmUserRepository::new(state.db.clone());

    let user_id = depot
        .get::<String>(DEPOT_KEY_ID)
        .map_err(|_| AppError::Unexpected(format!("Failed to obtain user id from depot")))?
        .to_owned();

    match FindUserByIdQuery::new(repository)
        .handle(FindUserById { id: user_id })
        .await
    {
        Ok(user) => {
            res.render(DataResponse::success(UserResource::from(user)));
            res.status_code(StatusCode::OK);
            Ok(())
        }
        Err(err) => Err(HttpError::InternalServerError(err.to_string())),
    }
}
