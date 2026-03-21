use std::sync::Arc;

use salvo::prelude::*;
use sea_orm::DatabaseConnection;

use crate::infrastructure::database::estabilish_connection;

#[derive(Default, Clone, Debug)]
pub struct State {
    pub db: Arc<DatabaseConnection>,
}

async fn create_app_state() -> Arc<State> {
    let connection = estabilish_connection().await;
    Arc::new(State {
        db: Arc::new(connection),
    })
}

fn create_router(state: Arc<State>) -> Router {
    Router::with_path("api").hoop(affix_state::inject(state))
}

pub async fn http_server_init() {
    tracing_subscriber::fmt::init();

    let acceptor = TcpListener::new("0.0.0.0:7231").bind().await;
    let state = create_app_state().await;
    let router = create_router(state);

    Server::new(acceptor).serve(router).await;
}
