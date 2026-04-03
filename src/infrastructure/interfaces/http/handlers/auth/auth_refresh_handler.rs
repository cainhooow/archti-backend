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
        .map_err(|_| HttpError::InternalServerError("Failed to obtain app state".to_string()))?;

    let cookie_service = &state.app.cookie_service;

    let refresh_token = req
        .parse_body::<RefreshRequest>()
        .await
        .and_then(|s| Ok(s.refresh_token.to_string()))
        .or_else(|_| {
            req.cookie(COOKIE_REFRESH_NAME)
                .map(|c| c.value().to_string())
                .ok_or(HttpError::BadRequest(
                    "refreshToken is required".to_string(),
                ))
        })?;

    let refreshed = state.app.identity.refresh_session(refresh_token).await?;

    _ = cookie_service.generate_sessions(&refreshed.access_token, &refreshed.refresh_token, res);

    res.status_code(StatusCode::OK);
    res.render(DataResponse {
        success: true,
        data: Some(RefreshResponse {
            access_token: refreshed.access_token.token,
            refresh_token: refreshed.refresh_token.token,
        }),
    });

    Ok(())
}
