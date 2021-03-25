use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

use super::Template;

static TEMPLATE: &str = "mod core;
mod features;

fn main() \\{}";

#[derive(Debug, Serialize)]
pub struct MainFileTemplate {}

impl MainFileTemplate {
    pub fn new() -> Self {
        Self {}
    }
}

impl Template for MainFileTemplate {
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
        let template = MainFileTemplate::new();

        let expect = "mod core;
mod features;

fn main() {}";

        let got = &template.render().unwrap();

        assert_eq!(expect, got)
    }
}
