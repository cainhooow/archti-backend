use std::sync::Arc;

use archti_backend::infrastructure::database::estabilish_connection;

use crate::commands::create_admin_user::CreateAdminUserCommand;
mod cli;
mod commands;

#[tokio::main]
async fn main() {
    _ = dotenvy::dotenv();

    let db = Arc::new(estabilish_connection().await);
    let args: Vec<String> = std::env::args().collect();

    let command = args.get(1).map(String::as_str);
    let _params = args.iter().skip(2).cloned().collect::<Vec<_>>();

    match command {
        Some("create-admin-user") => {
            if let Err(err) = CreateAdminUserCommand::new(db).handle().await {
                eprintln!("Error: {err}");
                std::process::exit(1);
            }
        }
        Some(other) => {
            eprintln!("Unknown command {other}");
            std::process::exit(1);
        }
        None => {
            eprintln!("No command provied");
            std::process::exit(1);
        }
    }
}
