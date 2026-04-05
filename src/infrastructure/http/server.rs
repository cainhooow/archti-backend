use std::{sync::Arc, time::Duration};

use salvo::cache::{Cache, MokaStore, RequestIssuer};
use salvo::logging::Logger as SalvoLogger;
use salvo::prelude::*;
use tokio::sync::mpsc;
use tracing_subscriber::EnvFilter;

use crate::{
    application::{events::IntegrationEvent, workers::notification_worker::notification_worker},
    infrastructure::{
        bootstrap::build_app_container,
        http::{middlewares::app_middleware::AppMiddleware, state::HttpState},
        http::routers::*,
    },
};

use super::middlewares::log_middleware::LogMiddleware;

fn create_router(state: Arc<HttpState>) -> Router {
    Router::with_path("api")
        .hoop(Cache::new(
            MokaStore::builder()
                .time_to_live(Duration::from_secs(60))
                .build(),
            RequestIssuer::default(),
        ))
        .hoop(Timeout::new(Duration::from_secs(40)))
        .hoop(affix_state::inject(state))
        .hoop(SalvoLogger::new())
        .hoop(AppMiddleware)
        .hoop(LogMiddleware)
        .push(v1::router())
}

pub async fn http_server_init() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "archgti_backend=debug,salvo=info".into()),
        )
        .init();

    let (tx, rx) = mpsc::unbounded_channel::<IntegrationEvent>();
    let app = build_app_container(tx).await;
    let handler = app.notifications.clone();
    let state = Arc::new(HttpState::new(app));

    tokio::spawn(async move { notification_worker(rx, handler).await });

    let acceptor = TcpListener::new("0.0.0.0:7231").bind().await;
    let router = create_router(state);

    Server::new(acceptor).serve(router).await;
}
