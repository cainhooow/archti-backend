use std::sync::Arc;

use archti_backend::infrastructure::database::estabilish_connection;

use crate::commands::create_permission::CreatePermissionCliCommand;
use crate::commands::create_admin_user::CreateAdminUserCommand;
use crate::commands::init_default_permissions::InitDefaultPermissionsCommand;
mod cli;
mod commands;

#[tokio::main]
async fn main() {
    _ = dotenvy::dotenv();

    let db = Arc::new(estabilish_connection().await);
    let args: Vec<String> = std::env::args().collect();

    let command = args.get(1).map(String::as_str);
    let params = args.iter().skip(2).cloned().collect::<Vec<_>>();

    match command {
        Some("create-admin-user") => {
            if let Err(err) = CreateAdminUserCommand::new(db).handle().await {
                eprintln!("Error: {err}");
                std::process::exit(1);
            }
        }
        Some("init-permissions") => {
            if let Err(err) = InitDefaultPermissionsCommand::new(db).handle().await {
                eprintln!("Error: {err}");
                std::process::exit(1);
            }
        }
        Some("create-permission") | Some("create-permissions") => {
            if let Err(err) = CreatePermissionCliCommand::new(db).handle(&params).await {
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
