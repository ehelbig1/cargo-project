use async_std::{
    fs::{DirBuilder, File, OpenOptions},
    io::prelude::WriteExt,
    path::Path,
};
use async_trait::async_trait;
use futures::try_join;
use std::io;
use std::process::Command;

#[async_trait]
pub trait Datasource {
    async fn create_git_repo(&self, project_name: &str) -> io::Result<()>;
    async fn create_gitignore(&self, project_name: &str, content: &[u8]) -> io::Result<()>;
    async fn create_cargo_file(&self, project_name: &str, content: &[u8]) -> io::Result<()>;
    async fn create_presentation_layer(
        &self,
        project_name: &str,
        main_file_content: &[u8],
    ) -> io::Result<()>;

    async fn create_domain_layer(
        &self,
        project_name: &str,
        main_file_content: &[u8],
    ) -> io::Result<()>;

    async fn create_data_layer(
        &self,
        project_name: &str,
        main_file_content: &[u8],
    ) -> io::Result<()>;
}

pub struct NewDatasource {}

impl NewDatasource {
    pub fn new() -> Self {
        Self {}
    }

    async fn create_module(&self, path: &Path) -> io::Result<()> {
        DirBuilder::new().create(path).await?;

        let path = format!(
            "{}/mod.rs",
            path.to_str()
                .expect(&format!("Error parsing path: {:?}", path))
        );
        let path = &Path::new(&path);
        File::create(path).await?;

        Ok(())
    }

    async fn update_main_file(&self, path: &Path, content: &[u8]) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .await?;

        file.write_all(content).await
    }
}

#[async_trait]
impl Datasource for NewDatasource {
    async fn create_git_repo(&self, project_name: &str) -> io::Result<()> {
        let output = Command::new("git").arg("init").arg(project_name).output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Error creating git repository for project: {}",
                    project_name
                ),
            ));
        }

        Ok(())
    }

    async fn create_gitignore(&self, project_name: &str, content: &[u8]) -> io::Result<()> {
        let path = format!("{}/.gitignore", project_name);
        let path = Path::new(&path);
        let mut file = File::create(path).await?;

        file.write_all(content).await
    }

    async fn create_cargo_file(&self, project_name: &str, content: &[u8]) -> io::Result<()> {
        let path = format!("{}/Cargo.toml", project_name);
        let path = Path::new(&path);
        let mut file = File::create(path).await?;

        file.write_all(content).await
    }

    async fn create_presentation_layer(
        &self,
        project_name: &str,
        main_file_content: &[u8],
    ) -> io::Result<()> {
        let path = format!("{}/{}", project_name, project_name);
        let path = Path::new(&path);
        let output = Command::new("cargo").arg("new").arg(path).output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error creating rust subproject: {}", project_name),
            ));
        }

        let path = format!("{}/{}/src/core", project_name, project_name);
        let path = Path::new(&path);
        let future_core_module = self.create_module(path);

        let path = format!("{}/{}/src/features", project_name, project_name);
        let path = Path::new(&path);
        let future_features_module = self.create_module(path);

        let path = format!("{}/{}/src/main.rs", project_name, project_name);
        let path = Path::new(&path);
        let future_main_file = self.update_main_file(path, main_file_content);

        try_join!(future_core_module, future_features_module, future_main_file)?;

        Ok(())
    }

    async fn create_domain_layer(
        &self,
        project_name: &str,
        main_file_content: &[u8],
    ) -> io::Result<()> {
        let path = format!("{}/{}-domain", project_name, project_name);
        let path = Path::new(&path);
        let output = Command::new("cargo")
            .arg("new")
            .arg(path)
            .arg("--lib")
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error creating rust subproject: {}", project_name),
            ));
        }

        let path = format!("{}/{}-domain/src/core", project_name, project_name);
        let path = Path::new(&path);
        let future_core_module = self.create_module(path);

        let path = format!("{}/{}-domain/src/features", project_name, project_name);
        let path = Path::new(&path);
        let future_features_module = self.create_module(path);

        let path = format!("{}/{}-domain/src/main.rs", project_name, project_name);
        let path = Path::new(&path);
        let future_main_file = self.update_main_file(path, main_file_content);

        try_join!(future_core_module, future_features_module, future_main_file)?;

        Ok(())
    }

    async fn create_data_layer(
        &self,
        project_name: &str,
        main_file_content: &[u8],
    ) -> io::Result<()> {
        let path = format!("{}/{}-data", project_name, project_name);
        let path = Path::new(&path);
        let output = Command::new("cargo")
            .arg("new")
            .arg(path)
            .arg("--lib")
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error creating rust subproject: {}", project_name),
            ));
        }

        let path = format!("{}/{}-data/src/core", project_name, project_name);
        let path = Path::new(&path);
        let future_core_module = self.create_module(path);

        let path = format!("{}/{}-data/src/features", project_name, project_name);
        let path = Path::new(&path);
        let future_features_module = self.create_module(path);

        let path = format!("{}/{}-data/src/main.rs", project_name, project_name);
        let path = Path::new(&path);
        let future_main_file = self.update_main_file(path, main_file_content);

        try_join!(future_core_module, future_features_module, future_main_file)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        assert_eq!(1, 1)
    }
}
