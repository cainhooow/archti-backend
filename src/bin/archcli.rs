use archti_backend::infrastructure::database::estabilish_connection;

mod commands;

#[tokio::main]
async fn main() {
    _ = dotenvy::dotenv();
    let db = estabilish_connection().await;
}
