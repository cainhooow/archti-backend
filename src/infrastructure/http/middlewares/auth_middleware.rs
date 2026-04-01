use std::sync::Arc;

use salvo::prelude::*;

use crate::infrastructure::{
    http::HttpState, interfaces::http::resources::DataResponse,
    services::cookie_service::COOKIE_SESSION_NAME,
};

pub const DEPOT_KEY_ID: &'static str = "user_id";
pub const DEPOT_KEY_AUTHORIZATION: &'static str = "authorization";

pub struct AuthMiddleware;

#[async_trait::async_trait]
impl Handler for AuthMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        if let Some(auth_header) = req.headers().get("Authorization") {
            let state = depot.obtain::<Arc<HttpState>>().unwrap();
            let auth_header = auth_header.to_str().unwrap_or("");

            let parts: Vec<&str> = auth_header.split(" ").collect();
            let token_type = parts.get(0).copied().unwrap_or("");
            if token_type != "Bearer" {
                res.render(DataResponse::error("Invalid Authorization Header"));
                res.status_code(StatusCode::UNAUTHORIZED);
                return;
            }

            let token = parts.get(1).copied().unwrap_or("");

            match state.app.auth_service.verify_token(token) {
                Ok(claims) => {
                    depot.insert(DEPOT_KEY_ID, claims);
                    depot.insert(DEPOT_KEY_AUTHORIZATION, String::from(token));
                    ctrl.call_next(req, depot, res).await;
                }
                Err(_) => {
                    res.render(DataResponse::error("Invalid Access Token"));
                    res.status_code(StatusCode::UNAUTHORIZED);
                }
            }
        } else if let Some(auth_cookie) = req.cookie(COOKIE_SESSION_NAME) {
            let token = auth_cookie.value();
            let state = depot.obtain::<Arc<HttpState>>().unwrap();

            match state.app.auth_service.verify_token(token) {
                Ok(claims) => {
                    depot.insert(DEPOT_KEY_ID, claims);
                    depot.insert(DEPOT_KEY_AUTHORIZATION, String::from(token));
                    ctrl.call_next(req, depot, res).await;
                }
                Err(_) => {
                    res.render(DataResponse::error("Invalid access token cookie"));
                    res.status_code(StatusCode::UNAUTHORIZED);
                }
            }
        } else {
            res.render(DataResponse::error("Access token absent"));
            res.status_code(StatusCode::UNAUTHORIZED);
        }
    }
}
