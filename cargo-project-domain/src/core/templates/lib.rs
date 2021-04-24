use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

use super::Template;

static TEMPLATE: &str = "pub mod core;
pub mod features;

#[cfg(test)]
mod tests \\{
    #[test]
    fn it_works() \\{
        assert_eq!(2 + 2, 4);
    }
}";

#[derive(Debug, Serialize)]
pub struct LibFileTemplate {}

impl LibFileTemplate {
    pub fn new() -> Self {
        Self {}
    }
}

impl Template for LibFileTemplate {
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
        let template = LibFileTemplate::new();

        let expect = "pub mod core;
pub mod features;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}";

        let got = &template.render().unwrap();

        assert_eq!(expect, got)
    }
}
