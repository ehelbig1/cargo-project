use async_trait::async_trait;
use project_data::features::add::datasource::{AddDatasource, Datasource};
use std::io;

use super::entities::{UpdateError, UpdateSuccess};

#[async_trait]
pub trait Repository {
    async fn update_cli(&self, name: &str, content: &[u8]) -> io::Result<()>;
    async fn update_domain(
        &self,
        name: &str,
        entities_file_content: &[u8],
        mod_file_content: &[u8],
        repository_file_content: &[u8],
        usecase_file_content: &[u8],
    ) -> io::Result<((), (), (), ())>;
    async fn update_data(
        &self,
        name: &str,
        datasource_file_content: &[u8],
        mod_file_content: &[u8],
        models_file_content: &[u8],
    ) -> io::Result<((), (), ())>;
}

pub struct AddRepository {
    datasource: Box<dyn Datasource + Send + Sync>,
}

impl AddRepository {
    pub fn new() -> Self {
        let datasource = Box::new(AddDatasource::new());
        Self { datasource }
    }
}

#[async_trait]
impl Repository for AddRepository {
    async fn update_cli(&self, name: &str, content: &[u8]) -> io::Result<()> {
        Ok(self.datasource.update_cli(name, content).await?)
    }

    async fn update_domain(
        &self,
        name: &str,
        entities_file_content: &[u8],
        mod_file_content: &[u8],
        repository_file_content: &[u8],
        usecase_file_content: &[u8],
    ) -> io::Result<((), (), (), ())> {
        Ok(self
            .datasource
            .update_domain(
                name,
                entities_file_content,
                mod_file_content,
                repository_file_content,
                usecase_file_content,
            )
            .await?)
    }

    async fn update_data(
        &self,
        name: &str,
        datasource_file_content: &[u8],
        mod_file_content: &[u8],
        models_file_content: &[u8],
    ) -> io::Result<((), (), ())> {
        Ok(self
            .datasource
            .update_data(
                name,
                datasource_file_content,
                mod_file_content,
                models_file_content,
            )
            .await?)
    }
}
