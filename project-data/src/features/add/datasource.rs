use async_std::prelude::*;
use async_std::{
    fs::{DirBuilder, File},
    io,
    path::Path,
};
use async_trait::async_trait;
use futures::try_join;

use super::models::{UpdateError, UpdateSuccess};

#[async_trait]
pub trait Datasource {
    async fn update_cli(&self, name: &str) -> Result<UpdateSuccess, UpdateError>;
    async fn update_domain(&self, name: &str) -> Result<UpdateSuccess, UpdateError>;
    async fn update_data(&self, name: &str) -> Result<UpdateSuccess, UpdateError>;
}

pub struct AddDatasource {}

impl AddDatasource {
    pub fn new() -> Self {
        Self {}
    }

    async fn create_and_write_file(&self, path: &Path, content: &[u8]) -> io::Result<String> {
        let mut file = File::create(path)
            .await
            .expect(&format!("Error creating file: {:?}", path));

        file.write_all(content)
            .await
            .expect(&format!("Error writing to file: {:?}", path));

        Ok(String::from("Success"))
    }
}

#[async_trait]
impl Datasource for AddDatasource {
    async fn update_cli(&self, name: &str) -> Result<UpdateSuccess, UpdateError> {
        let path = format!("project-cli/src/features/{}.rs", name);
        let path = Path::new(&path);
        let mut file = File::create(path)
            .await
            .expect(&format!("Error creating file: {}.rs", name));

        file.write_all(b"Hello, world!")
            .await
            .expect(&format!("Error writing to file: {}.rs", name));

        Ok(UpdateSuccess::new(format!(
            "Successfully created and wrote to file: {}.rs",
            name
        )))
    }

    async fn update_domain(&self, name: &str) -> Result<UpdateSuccess, UpdateError> {
        let path = format!("project-domain/src/features/{}", name);
        let path = Path::new(&path);

        DirBuilder::new()
            .create(path)
            .await
            .expect(&format!("Error creating directory: {:?}", path));

        let path = format!("project-domain/src/features/{}/entities.rs", name);
        let path = Path::new(&path);
        let future_entities_file = self.create_and_write_file(path, b"Hello, world!");

        let path = format!("project-domain/src/features/{}/mod.rs", name);
        let path = Path::new(&path);
        let future_mod_file = self.create_and_write_file(path, b"Hello, world!");

        let path = format!("project-domain/src/features/{}/repository.rs", name);
        let path = Path::new(&path);
        let future_repository_file = self.create_and_write_file(path, b"Hello, world!");

        let path = format!("project-domain/src/features/{}/usecase.rs", name);
        let path = Path::new(&path);
        let future_usecase_file = self.create_and_write_file(path, b"Hello, world!");

        let result = try_join!(
            future_entities_file,
            future_mod_file,
            future_repository_file,
            future_usecase_file
        );

        match result {
            Ok(_) => {
                return Ok(UpdateSuccess::new(String::from(
                    "Successfully created and wrote all files",
                )))
            }
            Err(error) => {
                return Err(UpdateError::new(format!(
                    "Failed to create and write files: {}",
                    error
                )))
            }
        }
    }

    async fn update_data(&self, name: &str) -> Result<UpdateSuccess, UpdateError> {
        let path = format!("project-data/src/features/{}", name);
        let path = Path::new(&path);

        DirBuilder::new()
            .create(path)
            .await
            .expect(&format!("Error creating directory: {:?}", path));

        let path = format!("project-data/src/features/{}/datasource.rs", name);
        let path = Path::new(&path);
        let future_datasource_file = self.create_and_write_file(path, b"Hello, world!");

        let path = format!("project-data/src/features/{}/mod.rs", name);
        let path = Path::new(&path);
        let future_mod_file = self.create_and_write_file(path, b"Hello, world!");

        let path = format!("project-data/src/features/{}/models.rs", name);
        let path = Path::new(&path);
        let future_models_file = self.create_and_write_file(path, b"Hello, world!");

        let result = try_join!(future_datasource_file, future_mod_file, future_models_file,);

        match result {
            Ok(_) => {
                return Ok(UpdateSuccess::new(String::from(
                    "Successfully created and wrote all files",
                )))
            }
            Err(error) => {
                return Err(UpdateError::new(format!(
                    "Failed to create and write files: {}",
                    error
                )))
            }
        }
    }
}
