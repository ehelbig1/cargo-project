use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

use super::Template;

static TEMPLATE: &str =
    "use {project}_data::features::{name}::datasource::\\{{name_title_case}Datasource, Datasource};

pub trait Repository \\{}

pub struct {name_title_case}Repository \\{
    datasource: Box<dyn Datasource + Send + Sync>,
}

impl {name_title_case}Repository \\{
    pub fn new() -> Self \\{
        let datasource = Box::new({name_title_case}Datasource::new());
        Self \\{ datasource }
    }
}

impl Repository for {name_title_case}Repository \\{}

#[cfg(test)]
mod tests \\{
    use super::*;

    struct MockDatasource \\{}

    impl Datasource for MockDatasource \\{}

    #[test]
    fn test() \\{
        assert_eq!(1, 1)
    }
}";

#[derive(Debug, Serialize)]
pub struct RepositoryTemplate<'a> {
    name: &'a str,
    name_title_case: &'a str,
    project: &'a str,
}

impl<'a> RepositoryTemplate<'a> {
    pub fn new(name: &'a str, name_title_case: &'a str, project: &'a str) -> Self {
        Self {
            name,
            name_title_case,
            project,
        }
    }
}

impl<'a> Template for RepositoryTemplate<'a> {
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
        let template = RepositoryTemplate::new("test", "Test", "test");

        let expect = "use test_data::features::test::datasource::{TestDatasource, Datasource};

pub trait Repository {}

pub struct TestRepository {
    datasource: Box<dyn Datasource + Send + Sync>,
}

impl TestRepository {
    pub fn new() -> Self {
        let datasource = Box::new(TestDatasource::new());
        Self { datasource }
    }
}

impl Repository for TestRepository {}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDatasource {}

    impl Datasource for MockDatasource {}

    #[test]
    fn test() {
        assert_eq!(1, 1)
    }
}";

        let got = &template.render().unwrap();

        assert_eq!(expect, got)
    }
}
