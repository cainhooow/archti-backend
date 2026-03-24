use crate::infrastructure::{
    database::estabilish_connection,
    interfaces::http::routers,
    services::{cookie_service::CookieService, jwt_auth_service::JwtAuthService},
};
use salvo::prelude::*;
use sea_orm::DatabaseConnection;
use std::{env, sync::Arc};

pub mod middlewares;

#[derive(Default, Clone, Debug)]
pub struct State {
    pub db: Arc<DatabaseConnection>,
    pub auth_service: Arc<JwtAuthService>,
    pub cookie_service: Arc<CookieService>,
}

async fn create_app_state() -> Arc<State> {
    let connection = estabilish_connection().await;
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_AUTH is not defined in .env");

    Arc::new(State {
        db: Arc::new(connection),
        auth_service: Arc::new(JwtAuthService::new(jwt_secret)),
        cookie_service: Arc::new(CookieService::new()),
    })
}

fn create_router(state: Arc<State>) -> Router {
    Router::with_path("api")
        .hoop(affix_state::inject(state))
        .push(routers::v1::router())
}

pub async fn http_server_init() {
    tracing_subscriber::fmt::init();

    let acceptor = TcpListener::new("0.0.0.0:7231").bind().await;
    let state = create_app_state().await;
    let router = create_router(state);

    Server::new(acceptor).serve(router).await;
}
