use std::sync::Arc;

use salvo::prelude::*;
use serde::Serialize;

use crate::infrastructure::{
    http::{State, middlewares::auth_middleware::DEPOT_KEY_ID},
    interfaces::http::{exceptions::HttpError, resources::DataResponse},
};

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

#[handler]
pub async fn auth_logout_handler(
    _req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| HttpError::InternalServerError(format!("Failed to obtain app state")))?;

    let _user_id = depot
        .get::<String>(DEPOT_KEY_ID)
        .map_err(|_| HttpError::InternalServerError(format!("Failed to obtain user id")))?;

    state.cookie_service.clear_sessions(res);

    res.render(DataResponse::success(Message {
        message: format!("Logged out successfully"),
    }));

    Ok(())
}
