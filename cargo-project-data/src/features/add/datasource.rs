use async_std::prelude::*;
use async_std::{
    fs::{DirBuilder, File, OpenOptions},
    io,
    path::Path,
};
use async_trait::async_trait;
use futures::try_join;

#[async_trait]
pub trait Datasource {
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

    async fn append_mod_file(&self, path: &Path, content: &[u8]) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .await
            .expect(&format!("Error writing to file: {:?}", path));

        file.write_all(content)
            .await
            .expect(&format!("Error writing to file: {:?}", path));

        Ok(())
    }
}

#[async_trait]
impl Datasource for AddDatasource {
    async fn update_cli(
        &self,
        project: &str,
        name: &str,
        cli_file_content: &[u8],
        parent_mod_file_content: &[u8],
    ) -> io::Result<()> {
        let path = format!("{}/src/features/mod.rs", project);
        let path = Path::new(&path);
        let future_parent_mod_file = self.append_mod_file(path, parent_mod_file_content);

        let path = format!("{}/src/features/{}.rs", project, name);
        let path = Path::new(&path);
        let future_cli_file = self.create_and_write_file(path, cli_file_content);

        try_join!(future_parent_mod_file, future_cli_file)?;

        Ok(())
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
        let path = format!("{}-domain/src/features/{}", project, name);
        let path = Path::new(&path);

        DirBuilder::new()
            .create(path)
            .await
            .expect(&format!("Error creating directory: {:?}", path));

        let path = format!("{}-domain/src/features/{}/entities.rs", project, name);
        let path = Path::new(&path);
        let future_entities_file = self.create_and_write_file(path, entities_file_content);

        let path = format!("{}-domain/src/features/{}/mod.rs", project, name);
        let path = Path::new(&path);
        let future_mod_file = self.create_and_write_file(path, mod_file_content);

        let path = format!("{}-domain/src/features/{}/repository.rs", project, name);
        let path = Path::new(&path);
        let future_repository_file = self.create_and_write_file(path, repository_file_content);

        let path = format!("{}-domain/src/features/{}/usecase.rs", project, name);
        let path = Path::new(&path);
        let future_usecase_file = self.create_and_write_file(path, usecase_file_content);

        let path = format!("{}-domain/src/features/mod.rs", project);
        let path = Path::new(&path);
        let future_parent_mod_file = self.append_mod_file(path, parent_mod_file_content);

        try_join!(
            future_entities_file,
            future_mod_file,
            future_repository_file,
            future_usecase_file,
            future_parent_mod_file,
        )?;

        Ok(())
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
        let path = format!("{}-data/src/features/{}", project, name);
        let path = Path::new(&path);

        DirBuilder::new()
            .create(path)
            .await
            .expect(&format!("Error creating directory: {:?}", path));

        let path = format!("{}-data/src/features/{}/datasource.rs", project, name);
        let path = Path::new(&path);
        let future_datasource_file = self.create_and_write_file(path, datasource_file_content);

        let path = format!("{}-data/src/features/{}/mod.rs", project, name);
        let path = Path::new(&path);
        let future_mod_file = self.create_and_write_file(path, mod_file_content);

        let path = format!("{}-data/src/features/{}/models.rs", project, name);
        let path = Path::new(&path);
        let future_models_file = self.create_and_write_file(path, models_file_content);

        let path = format!("{}-data/src/features/mod.rs", project);
        let path = Path::new(&path);
        let future_parent_mod_file = self.append_mod_file(path, parent_mod_file_content);

        try_join!(
            future_datasource_file,
            future_mod_file,
            future_models_file,
            future_parent_mod_file
        )?;

        Ok(())
    }
}
