use super::database::estabilish_connection;
use super::security::Argon2HasherImpl;
use super::services::{cookie_service::CookieService, jwt_auth_service::JwtAuthService};
use super::http::middlewares::app_middleware::AppMiddleware;
use super::interfaces::http::routers::*;

use salvo::prelude::*;
use sea_orm::DatabaseConnection;
use std::{env, sync::Arc};

pub mod middlewares;

#[derive(Default, Clone, Debug)]
pub struct State {
    pub db: Arc<DatabaseConnection>,
    pub hasher: Arc<Argon2HasherImpl>,
    pub auth_service: Arc<JwtAuthService>,
    pub cookie_service: Arc<CookieService>,
}

async fn create_app_state() -> Arc<State> {
    let connection = estabilish_connection().await;
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_AUTH is not defined in .env");
    Arc::new(State {
        db: Arc::new(connection),
        hasher: Arc::new(Argon2HasherImpl::default()),
        auth_service: Arc::new(JwtAuthService::new(jwt_secret)),
        cookie_service: Arc::new(CookieService::new()),
    })
}

fn create_router(state: Arc<State>) -> Router {
    Router::with_path("api")
        .hoop(affix_state::inject(state))
        .hoop(Logger::new())
        .hoop(AppMiddleware)
        .push(v1::router())
}

pub async fn http_server_init() {
    tracing_subscriber::fmt::init();

    let acceptor = TcpListener::new("0.0.0.0:7231").bind().await;
    let state = create_app_state().await;
    let router = create_router(state);

    Server::new(acceptor).serve(router).await;
}
