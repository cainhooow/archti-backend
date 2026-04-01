use std::sync::Arc;

use salvo::prelude::*;

use crate::{
    infrastructure::{
        http::{State, middlewares::auth_middleware::DEPOT_KEY_ID},
        interfaces::http::{
            exceptions::HttpError,
            resources::{DataResponse, user_resources::UserResource},
        },
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
        .map_err(|_| HttpError::InternalServerError(format!("Failed to obtain app state")))?;

    let user_id = depot
        .get::<String>(DEPOT_KEY_ID)
        .map_err(|_| {
            HttpError::InternalServerError(format!("Failed to obtain user id from depot"))
        })?
        .to_owned();

    let user = state.identity.current_user(user_id).await?;

    res.render(DataResponse::success(UserResource::from(user)));
    res.status_code(StatusCode::OK);
    Ok(())
}
