use async_trait::async_trait;
use futures::try_join;
use std::path::Path;

use super::repository::{NewRepository, Repository};

use crate::core::templates::cargo::CargoTemplate;
use crate::core::templates::lib::LibFileTemplate;
use crate::core::templates::main::MainFileTemplate;
use crate::core::templates::Template;

#[async_trait]
pub trait Usecase {
    async fn create_project(&self) -> String;
}

pub struct NewUsecase<'a> {
    project_name: &'a str,
    repository: Box<dyn Repository + Send + Sync + 'a>,
}

impl<'a> NewUsecase<'a> {
    pub fn new(project_name: &'a str, project_path: &'a Path) -> Self {
        let repository = Box::new(NewRepository::new(project_name, project_path));
        Self {
            project_name,
            repository,
        }
    }
}

#[async_trait]
impl<'a> Usecase for NewUsecase<'a> {
    async fn create_project(&self) -> String {
        self.repository.create_git_repo().await.expect(&format!(
            "Error creating git repo for: {}",
            self.project_name
        ));

        let future_gitignore = self.repository.create_gitignore(b"/target");

        let cargo_file_content = CargoTemplate::new(self.project_name)
            .render()
            .expect("Error rendering Cargo template");
        let future_cargo_file = self
            .repository
            .create_cargo_file(cargo_file_content.as_bytes());

        let main_file_content = MainFileTemplate::new()
            .render()
            .expect("Error rendering MainFile template");
        let future_presentation_layer = self
            .repository
            .create_presentation_layer(main_file_content.as_bytes());

        let lib_file_content = LibFileTemplate::new()
            .render()
            .expect("Error rendering LibFile template");
        let future_domain_layer = self
            .repository
            .create_domain_layer(lib_file_content.as_bytes());
        let future_data_layer = self
            .repository
            .create_data_layer(lib_file_content.as_bytes());

        try_join!(
            future_gitignore,
            future_cargo_file,
            future_presentation_layer,
            future_domain_layer,
            future_data_layer
        )
        .expect(&format!("Error creating project: {}", self.project_name));

        format!("Successfully created project: {}", self.project_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRepository {}

    #[async_trait]
    impl Repository for MockRepository {
        async fn create_git_repo(&self) -> std::io::Result<()> {
            Ok(())
        }

        async fn create_gitignore(&self, _content: &[u8]) -> std::io::Result<()> {
            Ok(())
        }

        async fn create_cargo_file(&self, _content: &[u8]) -> std::io::Result<()> {
            Ok(())
        }

        async fn create_presentation_layer(
            &self,
            _main_file_content: &[u8],
        ) -> std::io::Result<()> {
            Ok(())
        }

        async fn create_domain_layer(&self, _main_file_content: &[u8]) -> std::io::Result<()> {
            Ok(())
        }

        async fn create_data_layer(&self, _main_file_content: &[u8]) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[async_std::test]
    async fn test_create_project() {
        let project_name = "test";
        let repository = Box::new(MockRepository {});
        let usecase = NewUsecase {
            project_name,
            repository,
        };

        let expect = String::from("Successfully created project: test");
        let got = usecase.create_project().await;

        assert_eq!(expect, got)
    }
}
