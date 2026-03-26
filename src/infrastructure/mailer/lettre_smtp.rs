use std::env;

use lettre::{
    Address,
    message::{Mailbox, SinglePart, header::ContentType},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};

use crate::domain::services::mailer_service::Mailer;

pub struct LettreSMTPMailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

pub struct MailerConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub starttls: bool,
    pub auth: bool,
}

impl LettreSMTPMailer {
    pub fn new(config: MailerConfig) -> Self {
        let mut transport_builder = if config.starttls {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
                .expect("Failed to configure SMTP STARTTLS transport")
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host)
        }
        .port(config.port);

        if config.auth && !config.user.trim().is_empty() {
            transport_builder =
                transport_builder.credentials(Credentials::new(config.user, config.password));
        }

        let transport = transport_builder.build();
        Self { transport }
    }
}

#[async_trait::async_trait]
impl Mailer for LettreSMTPMailer {
    async fn send(&self, to: &str, subject: &str, body: String) -> Result<(), String> {
        let mail_from_name =
            env::var("SMTP_MAIL_FROM").or_else(|_| env::var("APP_NAME")).map_err(|_| {
                "SMTP_MAIL_FROM or APP_NAME must be defined in .env".to_string()
            })?;

        let mail_from = env::var("SMTP_USERNAME")
            .map_err(|_| "SMTP_USERNAME is not defined in .env".to_string())?;

        let from_mailbox = Mailbox::new(
            Some(mail_from_name),
            mail_from
                .parse::<Address>()
                .map_err(|err| err.to_string())?,
        );

        let mail = Message::builder()
            .from(from_mailbox)
            .to(to.parse::<Mailbox>().map_err(|err| err.to_string())?)
            .subject(subject)
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(body),
            )
            .map_err(|e| e.to_string())?;

        self.transport.send(mail).await.map_err(|e| e.to_string())?;

        Ok(())
    }
}
