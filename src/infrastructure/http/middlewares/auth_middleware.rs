use std::sync::Arc;

use salvo::prelude::*;

use crate::infrastructure::{
    http::State, interfaces::http::resources::DataResponse,
    services::cookie_service::COOKIE_SESSION_NAME,
};

pub const DEPOT_KEY_ID: &'static str = "user_id";

pub struct AuthMiddleware {}

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
            let state = depot.obtain::<Arc<State>>().unwrap();
            let auth_header = auth_header.to_str().unwrap_or("");

            match state.auth_service.validate_from_authorization(auth_header) {
                Ok(auth) => match state.auth_service.validate_token(auth.token) {
                    Ok(claims) => {
                        depot.insert(DEPOT_KEY_ID, claims.sub);
                        ctrl.call_next(req, depot, res).await;
                    }
                    Err(_) => {
                        res.render(DataResponse::error("Invalid Access Token"));
                        res.status_code(StatusCode::UNAUTHORIZED);
                    }
                },
                Err(_) => {
                    res.render(DataResponse::error("Invalid Access Token"));
                    res.status_code(StatusCode::UNAUTHORIZED);
                }
            }
        } else if let Some(auth_cookie) = req.cookie(COOKIE_SESSION_NAME) {
            let token = auth_cookie.value();
            let state = depot.obtain::<Arc<State>>().unwrap();

            match state.auth_service.validate_token(token) {
                Ok(claims) => {
                    depot.insert(DEPOT_KEY_ID, claims.sub);
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
