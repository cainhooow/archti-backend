use std::sync::Arc;

use crate::infrastructure::{
    http::{HttpState, middlewares::auth_middleware::DEPOT_KEY_ID},
    services::cookie_service::COOKIE_SESSION_NAME,
};
use salvo::{
    http::{
        HeaderMap, HeaderValue,
        header::{ACCEPT, CONTENT_TYPE, FORWARDED},
    },
    prelude::*,
};

pub struct AppMiddleware;

#[async_trait::async_trait]
impl Handler for AppMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let state = depot.obtain::<Arc<HttpState>>().unwrap().clone();

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

        let mut headers = HeaderMap::new();
        _ = req.add_header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
            true,
        );
        _ = req.add_header(ACCEPT, HeaderValue::from_static("application/json"), true);
        headers.insert(FORWARDED, HeaderValue::from_static("ArchTI"));
        res.set_headers(headers);

        if let Some(token_str) = token {
            if let Ok(claims) = state.app.auth_service.verify_token(&token_str) {
                depot.insert(DEPOT_KEY_ID, claims);
            } else {
                res.status_code(StatusCode::UNAUTHORIZED);
                _ = ctrl.call_next(req, depot, res).await;
                return;
            }
        }

        _ = ctrl.call_next(req, depot, res).await;
    }
}
