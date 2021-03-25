use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

use super::Template;

static TEMPLATE: &str = "[workspace]
members = [
    \"{project}\",
    \"{project}-domain\",
    \"{project}-data\"
]";

#[derive(Debug, Serialize)]
pub struct CargoTemplate<'a> {
    project: &'a str,
}

impl<'a> CargoTemplate<'a> {
    pub fn new(project: &'a str) -> Self {
        Self { project }
    }
}

impl<'a> Template for CargoTemplate<'a> {
    fn render(&self) -> Result<String, Box<dyn Error>> {
        let mut template = TinyTemplate::new();
        template.add_template("cargo", TEMPLATE)?;

        let rendered = template.render("cargo", self)?;

        Ok(rendered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let template = CargoTemplate::new("test");

        let expect = "[workspace]
members = [
    \"test\",
    \"test-domain\",
    \"test-data\"
]";

        let got = &template.render().unwrap();

        assert_eq!(expect, got)
    }
}
