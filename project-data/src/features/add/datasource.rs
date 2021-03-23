use async_std::prelude::*;
use async_std::{
    fs::{DirBuilder, File},
    io,
    path::Path,
};
use async_trait::async_trait;
use futures::try_join;

#[async_trait]
pub trait Datasource {
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

pub struct AddDatasource {}

impl AddDatasource {
    pub fn new() -> Self {
        Self {}
    }

    async fn create_and_write_file(&self, path: &Path, content: &[u8]) -> io::Result<()> {
        let mut file = File::create(path)
            .await
            .expect(&format!("Error creating file: {:?}", path));

        file.write_all(content)
            .await
            .expect(&format!("Error writing to file: {:?}", path));

        Ok(())
    }
}

#[async_trait]
impl Datasource for AddDatasource {
    async fn update_cli(&self, name: &str, content: &[u8]) -> io::Result<()> {
        let path = format!("project-cli/src/features/{}.rs", name);
        let path = Path::new(&path);

        Ok(self.create_and_write_file(path, content).await?)
    }

    async fn update_domain(
        &self,
        name: &str,
        entities_file_content: &[u8],
        mod_file_content: &[u8],
        repository_file_content: &[u8],
        usecase_file_content: &[u8],
    ) -> io::Result<((), (), (), ())> {
        let path = format!("project-domain/src/features/{}", name);
        let path = Path::new(&path);

        DirBuilder::new()
            .create(path)
            .await
            .expect(&format!("Error creating directory: {:?}", path));

        let path = format!("project-domain/src/features/{}/entities.rs", name);
        let path = Path::new(&path);
        let future_entities_file = self.create_and_write_file(path, entities_file_content);

        let path = format!("project-domain/src/features/{}/mod.rs", name);
        let path = Path::new(&path);
        let future_mod_file = self.create_and_write_file(path, mod_file_content);

        let path = format!("project-domain/src/features/{}/repository.rs", name);
        let path = Path::new(&path);
        let future_repository_file = self.create_and_write_file(path, repository_file_content);

        let path = format!("project-domain/src/features/{}/usecase.rs", name);
        let path = Path::new(&path);
        let future_usecase_file = self.create_and_write_file(path, usecase_file_content);

        Ok(try_join!(
            future_entities_file,
            future_mod_file,
            future_repository_file,
            future_usecase_file
        )?)
    }

    async fn update_data(
        &self,
        name: &str,
        datasource_file_content: &[u8],
        mod_file_content: &[u8],
        models_file_content: &[u8],
    ) -> io::Result<((), (), ())> {
        let path = format!("project-data/src/features/{}", name);
        let path = Path::new(&path);

        DirBuilder::new()
            .create(path)
            .await
            .expect(&format!("Error creating directory: {:?}", path));

        let path = format!("project-data/src/features/{}/datasource.rs", name);
        let path = Path::new(&path);
        let future_datasource_file = self.create_and_write_file(path, datasource_file_content);

        let path = format!("project-data/src/features/{}/mod.rs", name);
        let path = Path::new(&path);
        let future_mod_file = self.create_and_write_file(path, mod_file_content);

        let path = format!("project-data/src/features/{}/models.rs", name);
        let path = Path::new(&path);
        let future_models_file = self.create_and_write_file(path, models_file_content);

        Ok(try_join!(
            future_datasource_file,
            future_mod_file,
            future_models_file,
        )?)
    }
}
