use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

use super::Template;

static TEMPLATE: &str = "{{ for resource in resources }}pub mod {resource};\n{{ endfor }}";

#[derive(Debug, Serialize)]
pub struct ModFileTemplate<'a> {
    resources: Vec<&'a str>,
}

impl<'a> ModFileTemplate<'a> {
    pub fn new(resources: Vec<&'a str>) -> Self {
        Self { resources }
    }
}

impl<'a> Template for ModFileTemplate<'a> {
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
        let template = ModFileTemplate::new(vec!["test", "test2", "test3"]);

        let expect = "pub mod test;\npub mod test2;\npub mod test3;\n";
        let got = &template.render().unwrap();

        assert_eq!(expect, got)
    }
}
