use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

use super::Template;

static TEMPLATE: &str = "pub trait Datasource \\{}

pub struct {name_title_case}Datasource \\{}

impl {name_title_case}Datasource \\{
    pub fn new() -> Self \\{
        Self \\{}
    }
}

impl Datasource for {name_title_case}Datasource \\{}

#[cfg(test)]
mod tests \\{
    use super::*;

    #[test]
    fn test() \\{
        assert_eq!(1, 1)
    }
}";

#[derive(Debug, Serialize)]
pub struct DatasourceTemplate<'a> {
    name_title_case: &'a str,
}

impl<'a> DatasourceTemplate<'a> {
    pub fn new(name_title_case: &'a str) -> Self {
        Self { name_title_case }
    }
}

impl<'a> Template for DatasourceTemplate<'a> {
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
        let template = DatasourceTemplate::new("Test");

        let expect = "pub trait Datasource {}

pub struct TestDatasource {}

impl TestDatasource {
    pub fn new() -> Self {
        Self {}
    }
}

impl Datasource for TestDatasource {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1)
    }
}";

        let got = &template.render().unwrap();

        assert_eq!(expect, got)
    }
}
