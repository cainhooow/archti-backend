use std::{env, path::PathBuf, sync::Arc};

use sea_orm::DatabaseConnection;
use tokio::sync::mpsc;

use crate::{
    application::{
        company::CompanyApplication,
        events::IntegrationEvent,
        handlers::NotificationHandler,
        identity::IdentityApplication,
        services::access_control_service::AccessControlService,
        ports::{
            document_encryption::DocumentEncryption, password_hasher::PasswordHasher,
            password_reset_token_service::PasswordResetTokenService, token_service::TokenService,
        },
    },
    infrastructure::{
        database::estabilish_connection,
        mailer::lettre_smtp::{LettreSMTPMailer, MailerConfig},
        persistence::{
            sea_orm_company_repository::SeaOrmCompanyRepository,
            sea_orm_membership_repository::SeaOrmMembershipRepository,
            sea_orm_permission_repository::SeaOrmPermissionRepository,
            sea_orm_role_repository::SeaOrmRoleRepository,
            sea_orm_user_repository::SeaOrmUserRepository,
        },
        renderer::{HandlebarsRenderer, InlineCssRenderer},
        security::{Argon2HasherImpl, document_encryption::AppDocumentEncryption},
        services::{
            cookie_service::CookieService, jwt_auth_service::JwtAuthService,
            password_reset_token_service::JwtPasswordResetTokenService,
        },
    },
};

pub type IdentityApp = IdentityApplication<Arc<SeaOrmUserRepository>>;
pub type AccessControlApp =
    AccessControlService<Arc<SeaOrmUserRepository>, Arc<SeaOrmMembershipRepository>>;
pub type CompanyApp = CompanyApplication<
    Arc<SeaOrmCompanyRepository>,
    Arc<SeaOrmMembershipRepository>,
    Arc<SeaOrmRoleRepository>,
    Arc<SeaOrmPermissionRepository>,
>;

pub struct AppContainer {
    pub db: Arc<DatabaseConnection>,
    pub crypto: Arc<dyn DocumentEncryption>,
    pub auth_service: Arc<dyn TokenService>,
    pub cookie_service: Arc<CookieService>,
    pub notifications: Arc<NotificationHandler>,
    pub identity: Arc<IdentityApp>,
    pub access_control: Arc<AccessControlApp>,
    pub company: Arc<CompanyApp>,
}

fn parse_bool_env(key: &str) -> Option<bool> {
    env::var(key).ok().map(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

pub async fn build_app_container(tx: mpsc::UnboundedSender<IntegrationEvent>) -> Arc<AppContainer> {
    let connection = estabilish_connection().await;
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_AUTH is not defined in .env");
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL is not defined in .env");

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

    let core_renderer = HandlebarsRenderer::new(template_path);
    let renderer = InlineCssRenderer::new(core_renderer);
    let notification_handler = NotificationHandler::new(Box::new(mailer), Box::new(renderer));
    let db = Arc::new(connection);
    let hasher: Arc<dyn PasswordHasher> = Arc::new(Argon2HasherImpl::default());
    let auth_service: Arc<dyn TokenService> = Arc::new(JwtAuthService::new(jwt_secret.clone()));
    let reset_token_service: Arc<dyn PasswordResetTokenService> =
        Arc::new(JwtPasswordResetTokenService::new(jwt_secret));

    let identity = Arc::new(IdentityApplication::new(
        Arc::new(SeaOrmUserRepository::new(db.clone())),
        hasher,
        auth_service.clone(),
        reset_token_service,
        tx,
        frontend_url,
    ));

    let company = Arc::new(CompanyApplication::new(
        Arc::new(SeaOrmCompanyRepository::new(db.clone())),
        Arc::new(SeaOrmMembershipRepository::new(db.clone())),
        Arc::new(SeaOrmRoleRepository::new(db.clone())),
        Arc::new(SeaOrmPermissionRepository::new(db.clone())),
    ));
    let access_control = Arc::new(AccessControlService::new(
        Arc::new(SeaOrmUserRepository::new(db.clone())),
        Arc::new(SeaOrmMembershipRepository::new(db.clone())),
    ));

    Arc::new(AppContainer {
        db,
        crypto: Arc::new(AppDocumentEncryption::default()),
        auth_service,
        cookie_service: Arc::new(CookieService::new()),
        notifications: Arc::new(notification_handler),
        identity,
        access_control,
        company,
    })
}
