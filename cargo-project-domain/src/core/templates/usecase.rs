use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

use super::Template;

static TEMPLATE: &str = "use super::repository::\\{{name_title_case}Repository, Repository};

pub trait Usecase \\{}

pub struct {name_title_case}Usecase \\{
    repository: Box<dyn Repository + Send + Sync>
}

impl {name_title_case}Usecase \\{
    pub fn new() -> Self \\{
        let repository = Box::new({name_title_case}Repository::new());
        Self \\{ repository }
    }
}

impl Usecase for {name_title_case}Usecase \\{}

#[cfg(test)]
mod tests \\{
    use super::*;

    struct MockRepository \\{}

    impl Repository for MockRepository \\{}

    #[test]
    fn test() \\{
        assert_eq!(1, 1)
    }
}";

#[derive(Debug, Serialize)]
pub struct UsecaseTemplate<'a> {
    name_title_case: &'a str,
}

impl<'a> UsecaseTemplate<'a> {
    pub fn new(name_title_case: &'a str) -> Self {
        Self { name_title_case }
    }
}

impl<'a> Template for UsecaseTemplate<'a> {
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
        let template = UsecaseTemplate::new("Test");

        let expect = "use super::repository::{TestRepository, Repository};

pub trait Usecase {}

pub struct TestUsecase {
    repository: Box<dyn Repository + Send + Sync>
}

impl TestUsecase {
    pub fn new() -> Self {
        let repository = Box::new(TestRepository::new());
        Self { repository }
    }
}

impl Usecase for TestUsecase {}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRepository {}

    impl Repository for MockRepository {}

    #[test]
    fn test() {
        assert_eq!(1, 1)
    }
}";

        let got = &template.render().unwrap();

        assert_eq!(expect, got)
    }
}
