use super::database::estabilish_connection;
use super::http::middlewares::app_middleware::AppMiddleware;
use super::interfaces::http::routers::*;
use super::security::Argon2HasherImpl;
use super::services::{cookie_service::CookieService, jwt_auth_service::JwtAuthService};
use crate::application::handlers::NotificationHandler;
use crate::application::ports::document_encryption::DocumentEncryption;
use crate::application::ports::password_hasher::PasswordHasher;
use crate::application::ports::token_service::TokenService;
use crate::application::workers::notification_worker::notification_worker;
use crate::domain::events::DomainEvents;
use crate::infrastructure::http::middlewares::log_middleware::LogMiddleware;
use crate::infrastructure::mailer::lettre_smtp::{LettreSMTPMailer, MailerConfig};
use crate::infrastructure::renderer::HandlebarsRenderer;
use crate::infrastructure::security::document_encryption::AppDocumentEncryption;

use salvo::logging::Logger as SalvoLogger;
use salvo::prelude::*;
use sea_orm::DatabaseConnection;
use std::{env, sync::Arc};
use tokio::sync::mpsc;

pub mod middlewares;

#[derive(Clone)]
pub struct State {
    pub db: Arc<DatabaseConnection>,
    pub hasher: Arc<dyn PasswordHasher>,
    pub crypto: Arc<dyn DocumentEncryption>,
    pub auth_service: Arc<dyn TokenService>,
    pub cookie_service: Arc<CookieService>,
    pub notifications: Arc<NotificationHandler>,
    pub sender: mpsc::UnboundedSender<DomainEvents>,
}

async fn create_app_state(tx: mpsc::UnboundedSender<DomainEvents>) -> Arc<State> {
    let connection = estabilish_connection().await;
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_AUTH is not defined in .env");

    let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST is not defined in .env");
    let smtp_port = env::var("SMTP_PORT").expect("SMTP_PORT is not defined in .env");
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME is not defined in .env");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not defined in .env");

    let mailer = LettreSMTPMailer::new(MailerConfig {
        server: smtp_host,
        user: smtp_username,
        password: smtp_password,
    });
    let renderer = HandlebarsRenderer::new("../templates");
    let notification_handler = NotificationHandler::new(Box::new(mailer), Box::new(renderer));

    Arc::new(State {
        db: Arc::new(connection),
        hasher: Arc::new(Argon2HasherImpl::default()),
        crypto: Arc::new(AppDocumentEncryption::default()),
        auth_service: Arc::new(JwtAuthService::new(jwt_secret)),
        cookie_service: Arc::new(CookieService::new()),
        notifications: Arc::new(notification_handler),
        sender: tx,
    })
}

fn create_router(state: Arc<State>) -> Router {
    Router::with_path("api")
        .hoop(affix_state::inject(state))
        .hoop(SalvoLogger::new())
        .hoop(AppMiddleware)
        .hoop(LogMiddleware)
        .push(v1::router())
}

pub async fn http_server_init() {
    tracing_subscriber::fmt::init();

    let (tx, rx) = mpsc::unbounded_channel::<DomainEvents>();
    let state = create_app_state(tx).await;
    // email worker
    let handler = state.notifications.clone();
    tokio::spawn(async move { notification_worker(rx, handler).await });

    let acceptor = TcpListener::new("0.0.0.0:7231").bind().await;
    let router = create_router(state);

    Server::new(acceptor).serve(router).await;
}
