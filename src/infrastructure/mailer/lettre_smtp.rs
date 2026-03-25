use std::env;

use lettre::{
    AsyncSmtpTransport, Message, Tokio1Executor, transport::smtp::authentication::Credentials,
};

pub struct LettreSMTPMailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

pub struct MailerConfig {
    pub server: String,
    pub user: String,
    pub password: String,
}

impl LettreSMTPMailer {
    pub fn new(config: MailerConfig) -> Self {
        let credentials = Credentials::new(config.user, config.password);
        let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.server)
            .unwrap()
            .credentials(credentials)
            .build();
        Self { transport }
    }
}

impl Mailer for LettreSMTPMailer {
    async fn send(&self, to: &str, subject: &str, body: &str) {
        // mail_from_name example: {app_name}
        let mail_from_name =
            env::var("SMTP_MAIL_FROM").expect("SMTP_MAIL_FROM is not defined in .env");
        // mail_from example: {app_name} <{smtp_username}>
        let mail_from = env::var("SMTP_USERNAME").expect("SMTP_USERNAME is not defined in .env");
        // mail_from example: {app_name} <{smtp_username}>
        let mail_from = format!("{} <{}>", mail_from_name, mail_from);
        // mail example: from: {app_name} <{smtp_username}>, to: {to}, subject: {subject}, body: {body}
        let mail = Message::builder()
            .from(mail_from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .body(body)
            .map_err(|e| e.to_string())?;
        self.transport.send(mail).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
