use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

use super::Template;

static TEMPLATE: &str = "";

#[derive(Debug, Serialize)]
pub struct EntitiesTemplate<'a> {
    name: &'a str,
}

impl<'a> EntitiesTemplate<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> Template for EntitiesTemplate<'a> {
    fn render(&self) -> Result<String, Box<dyn Error>> {
        let mut template = TinyTemplate::new();
        template.add_template("cli", TEMPLATE)?;

        let rendered = template.render("cli", self)?;

        Ok(rendered)
    }
}
