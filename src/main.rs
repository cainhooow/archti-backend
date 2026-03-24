use crate::infrastructure::http::http_server_init;

mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    _ = dotenvy::dotenv();
    http_server_init().await;
}
