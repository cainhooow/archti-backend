pub trait TemplateRenderer: Send + Sync {
    fn render(&self, template_name: &str, data: serde_json::Value) -> Result<String, String>;
}
