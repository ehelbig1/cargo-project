use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

use super::Template;

static TEMPLATE: &str =
    "use project_domain::features::{name}::usecase::\\{{name_title_case}Usecase, Usecase};";

#[derive(Debug, Serialize)]
pub struct CliTemplate<'a> {
    name: &'a str,
    name_title_case: &'a str,
}

impl<'a> CliTemplate<'a> {
    pub fn new(name: &'a str, name_title_case: &'a str) -> Self {
        Self {
            name,
            name_title_case,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let template = CliTemplate::new("test", "Test");

        let expect = "use project_domain::features::test::usecase::{TestUsecase, Usecase};";
        let got = &template.render().unwrap();

        assert_eq!(expect, got)
    }
}
