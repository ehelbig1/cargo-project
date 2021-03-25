use async_trait::async_trait;
use cargo_project_data::features::add::datasource::{AddDatasource, Datasource};
use std::io;

#[async_trait]
pub trait Repository {
    async fn update_cli(
        &self,
        project: &str,
        name: &str,
        cli_file_content: &[u8],
        parent_mod_file_content: &[u8],
    ) -> io::Result<()>;
    async fn update_domain(
        &self,
        project: &str,
        name: &str,
        entities_file_content: &[u8],
        mod_file_content: &[u8],
        repository_file_content: &[u8],
        usecase_file_content: &[u8],
        parent_mod_file_content: &[u8],
    ) -> io::Result<()>;
    async fn update_data(
        &self,
        project: &str,
        name: &str,
        datasource_file_content: &[u8],
        mod_file_content: &[u8],
        models_file_content: &[u8],
        parent_mod_file_content: &[u8],
    ) -> io::Result<()>;
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
    async fn update_cli(
        &self,
        project: &str,
        name: &str,
        cli_file_content: &[u8],
        parent_mod_file_content: &[u8],
    ) -> io::Result<()> {
        Ok(self
            .datasource
            .update_cli(project, name, cli_file_content, parent_mod_file_content)
            .await?)
    }

    async fn update_domain(
        &self,
        project: &str,
        name: &str,
        entities_file_content: &[u8],
        mod_file_content: &[u8],
        repository_file_content: &[u8],
        usecase_file_content: &[u8],
        parent_mod_file_content: &[u8],
    ) -> io::Result<()> {
        Ok(self
            .datasource
            .update_domain(
                project,
                name,
                entities_file_content,
                mod_file_content,
                repository_file_content,
                usecase_file_content,
                parent_mod_file_content,
            )
            .await?)
    }

    async fn update_data(
        &self,
        project: &str,
        name: &str,
        datasource_file_content: &[u8],
        mod_file_content: &[u8],
        models_file_content: &[u8],
        parent_mod_file_content: &[u8],
    ) -> io::Result<()> {
        Ok(self
            .datasource
            .update_data(
                project,
                name,
                datasource_file_content,
                mod_file_content,
                models_file_content,
                parent_mod_file_content,
            )
            .await?)
    }
}
