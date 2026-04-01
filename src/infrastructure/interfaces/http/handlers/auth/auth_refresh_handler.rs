use std::sync::Arc;

use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use crate::infrastructure::{
    http::HttpState,
    interfaces::http::{exceptions::HttpError, resources::DataResponse},
    services::cookie_service::COOKIE_REFRESH_NAME,
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
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<Arc<HttpState>>()
        .map_err(|_| HttpError::InternalServerError(format!("Failed to obtain app state")))?;

    let cookie_service = state.app.cookie_service.clone();

    if let Some(refresh_cookie) = req.cookies().get(COOKIE_REFRESH_NAME) {
        let refreshed = state
            .app
            .identity
            .refresh_session(refresh_cookie.value().to_string())
            .await?;

        _ = cookie_service.generate_sessions(
            &refreshed.access_token,
            &refreshed.refresh_token,
            res,
        );

        res.status_code(StatusCode::OK);
        res.render(DataResponse {
            success: true,
            data: Some(RefreshResponse {
                access_token: refreshed.access_token.token,
                refresh_token: refreshed.refresh_token.token,
            }),
        });
        return Ok(());
    } else {
        match req.parse_body::<RefreshRequest>().await {
            Ok(request) => {
                let refreshed = state
                    .app
                    .identity
                    .refresh_session(request.refresh_token)
                    .await?;

                _ = state.app.cookie_service.generate_sessions(
                    &refreshed.access_token,
                    &refreshed.refresh_token,
                    res,
                );

                res.status_code(StatusCode::OK);
                res.render(DataResponse {
                    success: true,
                    data: Some(RefreshResponse {
                        access_token: refreshed.access_token.token,
                        refresh_token: refreshed.refresh_token.token,
                    }),
                });
                return Ok(());
            }
            Err(err) => {
                return Err(HttpError::BadRequest(err.to_string()));
            }
        }
    }
}
