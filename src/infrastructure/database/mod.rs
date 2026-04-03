use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn estabilish_connection() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL NOT PROVIDED");

    match Database::connect(&database_url).await {
        Ok(connection) => connection,
        Err(err) => {
            panic!("Failed to connect database {:}", err);
        }
    }
}
