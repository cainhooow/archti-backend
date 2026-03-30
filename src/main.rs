use archti_backend::infrastructure::http::http_server_init;

#[tokio::main]
async fn main() {
    _ = dotenvy::dotenv();
    http_server_init().await;
}