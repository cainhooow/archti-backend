use std::sync::Arc;

use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    application::exceptions::{AppError, AppResult},
    infrastructure::{
        http::{State},
        interfaces::http::resources::DataResponse,
        services::cookie_service::COOKIE_REFRESH_NAME,
    },
};

#[derive(Serialize)]
pub struct RefreshResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshRequest {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[handler]
pub async fn auth_refresh_handler(
    res: &mut Response,
    depot: &mut Depot,
    req: &mut Request,
) -> AppResult<()> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| AppError::Unexpected(format!("Failed to obtain app state")))?;

    let auth_service = state.auth_service.clone();
    let cookie_service = state.cookie_service.clone();

    if let Some(refresh_cookie) = req.cookies().get(COOKIE_REFRESH_NAME) {
        let token = refresh_cookie.value();
        let user_id = auth_service.get_refresh_sub(token)?;

        let new_access = auth_service.renew_token(token)?;
        let new_refresh = auth_service.generate_refresh_token(&user_id)?;

        _ = cookie_service.generate_sessions(&new_access, &new_refresh, res);

        res.status_code(StatusCode::OK);
        res.render(DataResponse {
            success: true,
            data: Some(RefreshResponse {
                access_token: new_access.token,
                refresh_token: new_refresh.token,
            }),
        });
        return Ok(());
    } else {
        match req.parse_body::<RefreshRequest>().await {
            Ok(request) => {
                let user_id = auth_service.get_refresh_sub(&request.refresh_token)?;

                let new_access = auth_service.renew_token(&request.refresh_token)?;
                let new_refresh = auth_service.generate_refresh_token(&user_id)?;

                _ = state
                    .cookie_service
                    .generate_sessions(&new_access, &new_refresh, res);

                res.status_code(StatusCode::OK);
                res.render(DataResponse {
                    success: true,
                    data: Some(RefreshResponse {
                        access_token: new_access.token,
                        refresh_token: new_refresh.token,
                    }),
                });
                return Ok(());
            }
            Err(err) => {
                return Err(AppError::Unexpected(format!(
                    "Failed to parse refresh request: {}",
                    err
                )));
            }
        }
    }
}
