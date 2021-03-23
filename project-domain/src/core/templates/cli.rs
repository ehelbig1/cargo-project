use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

use super::Template;

static TEMPLATE: &str = "Hello {name}!";

#[derive(Debug, Serialize)]
pub struct CliTemplate<'a> {
    name: &'a str,
}

impl<'a> CliTemplate<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> Template for CliTemplate<'a> {
    fn render(&self) -> Result<String, Box<dyn Error>> {
        let mut template = TinyTemplate::new();
        template.add_template("cli", TEMPLATE)?;

        let rendered = template.render("cli", self)?;

        Ok(rendered)
    }
}
