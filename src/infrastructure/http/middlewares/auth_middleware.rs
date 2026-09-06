use std::sync::Arc;

use salvo::prelude::*;

use crate::infrastructure::{
    http::HttpState, http::resources::DataResponse, services::cookie_service::COOKIE_SESSION_NAME,
};

pub const DEPOT_KEY_ID: &str = "user_id";
pub const DEPOT_KEY_AUTHORIZATION: &str = "authorization";

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
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| {
                let parts: Vec<&str> = s.split_whitespace().collect();
                if parts.first() == Some(&"Bearer") {
                    parts.get(1).map(|&t| t.to_string())
                } else {
                    None
                }
            })
            .or_else(|| {
                req.cookie(COOKIE_SESSION_NAME)
                    .map(|c| c.value().to_string())
            });

        if let Some(token_str) = token {
            let state = depot.obtain::<Arc<HttpState>>().unwrap();

            match state.app.auth_service.verify_token(&token_str) {
                Ok(claims) => {
                    depot.insert(DEPOT_KEY_ID, claims);
                    depot.insert(DEPOT_KEY_AUTHORIZATION, token_str);
                    ctrl.call_next(req, depot, res).await;
                }
                Err(_) => {
                    res.render(DataResponse::error("Invalid Access Token"));
                    res.status_code(StatusCode::UNAUTHORIZED);
                }
            }
        } else {
            res.render(DataResponse::error("Access token absent"));
            res.status_code(StatusCode::UNAUTHORIZED);
        }
    }
}
