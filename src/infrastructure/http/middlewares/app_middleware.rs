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

        let mut headers = HeaderMap::new();
        _ = req.add_header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
            true,
        );
        _ = req.add_header(ACCEPT, HeaderValue::from_static("application/json"), true);
        headers.insert(FORWARDED, HeaderValue::from_static("ArchTI"));

        if let Some(auth_header) = req.headers().clone().get("Authorization") {
            let auth_header = auth_header.to_str().unwrap_or("");
            let parts: Vec<&str> = auth_header.split(" ").collect();

            let token_typ = parts.get(0).copied().unwrap();
            if token_typ != "Bearer" {
                res.set_headers(headers.clone());
                _ = ctrl.call_next(req, depot, res).await;
            }

            let token = parts.get(1).copied().unwrap_or("");
            match state.app.auth_service.verify_token(token) {
                Ok(claims) => {
                    depot.insert(DEPOT_KEY_ID, claims);
                    res.set_headers(headers);
                    _ = ctrl.call_next(req, depot, res).await;
                }
                Err(err) => {
                    println!("Error in token verifier: {:?}", err);
                    res.set_headers(headers);
                    _ = ctrl.call_next(req, depot, res).await;
                }
            }
        } else if let Some(auth_cookie) = req.cookie(COOKIE_SESSION_NAME) {
            let token = auth_cookie.value();
            match state.app.auth_service.verify_token(token) {
                Ok(claims) => {
                    depot.insert(DEPOT_KEY_ID, claims);
                    res.set_headers(headers);
                    _ = ctrl.call_next(req, depot, res).await;
                }
                Err(err) => {
                    println!("Error in token verifier: {:?}", err);
                    res.set_headers(headers);
                    _ = ctrl.call_next(req, depot, res).await;
                }
            }
        } else {
            res.set_headers(headers);
            _ = ctrl.call_next(req, depot, res).await;
        };
    }
}
