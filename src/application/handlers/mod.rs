use crate::{
    application::ports::{mailer::Mailer, template_renderer::TemplateRenderer},
    domain::notifications::EmailMessage,
};

pub struct NotificationHandler {
    mailer: Box<dyn Mailer>,
    renderer: Box<dyn TemplateRenderer>,
}

impl NotificationHandler {
    pub fn new(mailer: Box<dyn Mailer>, renderer: Box<dyn TemplateRenderer>) -> Self {
        Self { mailer, renderer }
    }
    
    pub async fn send<T: EmailMessage>(&self, to: &str, message: T) -> Result<(), String> {
        let body = self.renderer.render(message.template(), message.data())?;
        self.mailer.send(to, message.subject(), body).await
    }
}
