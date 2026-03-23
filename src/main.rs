use crate::infrastructure::http::http_server_init;

mod infrastructure;
mod application;
mod domain;

#[tokio::main]
async fn main() {
    _ = dotenvy::dotenv();
    http_server_init().await;
}
