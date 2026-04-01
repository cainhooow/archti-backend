use super::database::estabilish_connection;
use super::http::middlewares::app_middleware::AppMiddleware;
use super::interfaces::http::routers::*;
use super::security::Argon2HasherImpl;
use super::services::{cookie_service::CookieService, jwt_auth_service::JwtAuthService};
use crate::application::events::IntegrationEvent;
use crate::application::handlers::NotificationHandler;
use crate::application::ports::document_encryption::DocumentEncryption;
use crate::application::ports::password_hasher::PasswordHasher;
use crate::application::ports::password_reset_token_service::PasswordResetTokenService;
use crate::application::ports::token_service::TokenService;
use crate::application::workers::notification_worker::notification_worker;
use crate::infrastructure::http::middlewares::log_middleware::LogMiddleware;
use crate::infrastructure::mailer::lettre_smtp::{LettreSMTPMailer, MailerConfig};
use crate::infrastructure::renderer::{HandlebarsRenderer, InlineCssRenderer};
use crate::infrastructure::security::document_encryption::AppDocumentEncryption;
use crate::infrastructure::services::password_reset_token_service::JwtPasswordResetTokenService;

use salvo::cache::{Cache, MokaStore, RequestIssuer};
use salvo::logging::Logger as SalvoLogger;
use salvo::prelude::*;
use sea_orm::DatabaseConnection;
use std::time::Duration;
use std::{env, path::PathBuf, sync::Arc};
use tokio::sync::mpsc;
use tracing_subscriber::EnvFilter;

pub mod middlewares;

#[derive(Clone)]
pub struct State {
    pub db: Arc<DatabaseConnection>,
    pub hasher: Arc<dyn PasswordHasher>,
    pub crypto: Arc<dyn DocumentEncryption>,
    pub auth_service: Arc<dyn TokenService>,
    pub reset_token_service: Arc<dyn PasswordResetTokenService>,
    pub cookie_service: Arc<CookieService>,
    pub notifications: Arc<NotificationHandler>,
    pub sender: mpsc::UnboundedSender<IntegrationEvent>,
}

fn parse_bool_env(key: &str) -> Option<bool> {
    env::var(key).ok().map(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

async fn create_app_state(tx: mpsc::UnboundedSender<IntegrationEvent>) -> Arc<State> {
    let connection = estabilish_connection().await;
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_AUTH is not defined in .env");
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());

    let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST is not defined in .env");
    let smtp_port = env::var("SMTP_PORT")
        .expect("SMTP_PORT is not defined in .env")
        .parse::<u16>()
        .expect("SMTP_PORT must be a valid port number");
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME is not defined in .env");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not defined in .env");
    let smtp_starttls = parse_bool_env("SMTP_STARTTLS").unwrap_or(app_env != "dev");
    let smtp_auth = parse_bool_env("SMTP_AUTH").unwrap_or(app_env != "dev");

    let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("application")
        .join("templates");

    let mailer = LettreSMTPMailer::new(MailerConfig {
        host: smtp_host,
        port: smtp_port,
        user: smtp_username,
        password: smtp_password,
        starttls: smtp_starttls,
        auth: smtp_auth,
    });

    // base renderer
    let core_renderer = HandlebarsRenderer::new(template_path);
    // inline CSS renderer
    let renderer = InlineCssRenderer::new(core_renderer);
    let notification_handler = NotificationHandler::new(Box::new(mailer), Box::new(renderer));

    Arc::new(State {
        db: Arc::new(connection),
        hasher: Arc::new(Argon2HasherImpl::default()),
        crypto: Arc::new(AppDocumentEncryption::default()),
        auth_service: Arc::new(JwtAuthService::new(jwt_secret.clone())),
        reset_token_service: Arc::new(JwtPasswordResetTokenService::new(jwt_secret)),
        cookie_service: Arc::new(CookieService::new()),
        notifications: Arc::new(notification_handler),
        sender: tx,
    })
}

fn create_router(state: Arc<State>) -> Router {
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
    let state = create_app_state(tx).await;
    // email worker
    let handler = state.notifications.clone();
    tokio::spawn(async move { notification_worker(rx, handler).await });

    let acceptor = TcpListener::new("0.0.0.0:7231").bind().await;
    let router = create_router(state);

    Server::new(acceptor).serve(router).await;
}
