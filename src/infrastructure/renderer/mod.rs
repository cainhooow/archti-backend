use std::path::Path;

use handlebars::{DirectorySourceOptionsBuilder, Handlebars};

use crate::domain::services::renderer_service::TemplateRenderer;

pub struct HandlebarsRenderer {
    registry: Handlebars<'static>,
}

impl HandlebarsRenderer {
    pub fn new<P: AsRef<Path>>(template_path: P) -> Self {
        let mut hb = Handlebars::new();
        let mut config = DirectorySourceOptionsBuilder::default();
        config.tpl_extension(".hbs");
        config.hidden(false);
        config.temporary(false);

        hb.register_templates_directory(template_path, config.build().unwrap())
            .expect("Failed to load templates");

        Self { registry: hb }
    }
}

impl TemplateRenderer for HandlebarsRenderer {
    fn render(&self, template_name: &str, data: serde_json::Value) -> Result<String, String> {
        self.registry
            .render(template_name, &data)
            .map_err(|e| e.to_string())
    }
}
