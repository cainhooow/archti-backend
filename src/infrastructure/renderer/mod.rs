use std::path::Path;

use handlebars::{DirectorySourceOptionsBuilder, Handlebars};

use crate::application::ports::template_renderer::TemplateRenderer;

pub struct HandlebarsRenderer {
    registry: Handlebars<'static>,
}

pub struct InlineCssRenderer<R> {
    inner: R,
}

impl<R> InlineCssRenderer<R> {
    pub fn new(inner: R) -> Self {
        Self { inner }
    }
}

impl<R> TemplateRenderer for InlineCssRenderer<R>
where
    R: TemplateRenderer,
{
    fn render(&self, template_name: &str, data: serde_json::Value) -> Result<String, String> {
        let html = self.inner.render(template_name, data)?;
        css_inline::inline(&html).map_err(|err| err.to_string())
    }
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
