use async_std::{
    fs::{DirBuilder, File, OpenOptions},
    io::prelude::WriteExt,
};
use async_trait::async_trait;
use futures::try_join;
use std::io;
use std::path::Path;
use std::process::Command;

#[async_trait]
pub trait Datasource {
    async fn create_git_repo(&self) -> io::Result<()>;
    async fn create_gitignore(&self, content: &[u8]) -> io::Result<()>;
    async fn create_cargo_file(&self, content: &[u8]) -> io::Result<()>;
    async fn create_presentation_layer(&self, main_file_content: &[u8]) -> io::Result<()>;
    async fn create_domain_layer(&self, main_file_content: &[u8]) -> io::Result<()>;
    async fn create_data_layer(&self, main_file_content: &[u8]) -> io::Result<()>;
}

pub struct NewDatasource<'a> {
    project_name: &'a str,
    project_path: &'a Path,
}

impl<'a> NewDatasource<'a> {
    pub fn new(project_name: &'a str, project_path: &'a Path) -> Self {
        Self {
            project_name,
            project_path,
        }
    }

    async fn create_module(&self, path: &Path) -> io::Result<()> {
        DirBuilder::new().create(path).await?;

        let path = format!(
            "{}/mod.rs",
            path.to_str()
                .expect(&format!("Error parsing path: {:?}", path))
        );
        let path = Path::new(&path);

        println!("{:?}", path);
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
impl<'a> Datasource for NewDatasource<'a> {
    async fn create_git_repo(&self) -> io::Result<()> {
        let path = format!(
            "{}/{}",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name
        );
        let path = Path::new(&path);
        let output = Command::new("git").arg("init").arg(path).output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Error creating git repository for project: {}",
                    self.project_name
                ),
            ));
        }

        Ok(())
    }

    async fn create_gitignore(&self, content: &[u8]) -> io::Result<()> {
        let path = format!(
            "{}/{}/.gitignore",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name
        );
        let path = Path::new(&path);
        let mut file = File::create(path).await?;

        file.write_all(content).await
    }

    async fn create_cargo_file(&self, content: &[u8]) -> io::Result<()> {
        let path = format!(
            "{}/{}/Cargo.toml",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name
        );
        let path = Path::new(&path);
        let mut file = File::create(path).await?;

        file.write_all(content).await
    }

    async fn create_presentation_layer(&self, main_file_content: &[u8]) -> io::Result<()> {
        let path = format!(
            "{}/{}/{}",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name,
            self.project_name,
        );

        let path = Path::new(&path);
        let output = Command::new("cargo").arg("new").arg(path).output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error creating rust subproject: {}", self.project_name),
            ));
        }

        let path = format!(
            "{}/{}/{}/src/core",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name,
            self.project_name,
        );
        let path = Path::new(&path);
        let future_core_module = self.create_module(path);

        let path = format!(
            "{}/{}/{}/src/features",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name,
            self.project_name
        );
        let path = Path::new(&path);
        let future_features_module = self.create_module(path);

        let path = format!(
            "{}/{}/{}/src/main.rs",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name,
            self.project_name
        );
        let path = Path::new(&path);
        let future_main_file = self.update_main_file(path, main_file_content);

        try_join!(future_core_module, future_features_module, future_main_file)?;

        Ok(())
    }

    async fn create_domain_layer(&self, main_file_content: &[u8]) -> io::Result<()> {
        let path = format!(
            "{}/{}/{}-domain",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name,
            self.project_name
        );
        let path = Path::new(&path);
        let output = Command::new("cargo")
            .arg("new")
            .arg(path)
            .arg("--lib")
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error creating rust subproject: {}", self.project_name),
            ));
        }

        let path = format!(
            "{}/{}/{}-domain/src/core",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name,
            self.project_name
        );
        let path = Path::new(&path);
        let future_core_module = self.create_module(path);

        let path = format!(
            "{}/{}/{}-domain/src/features",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name,
            self.project_name
        );
        let path = Path::new(&path);
        let future_features_module = self.create_module(path);

        let path = format!(
            "{}/{}/{}-domain/src/lib.rs",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name,
            self.project_name
        );
        let path = Path::new(&path);
        let future_main_file = self.update_main_file(path, main_file_content);

        try_join!(future_core_module, future_features_module, future_main_file)?;

        Ok(())
    }

    async fn create_data_layer(&self, main_file_content: &[u8]) -> io::Result<()> {
        let path = format!(
            "{}/{}/{}-data",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name,
            self.project_name
        );
        let path = Path::new(&path);
        let output = Command::new("cargo")
            .arg("new")
            .arg(path)
            .arg("--lib")
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error creating rust subproject: {}", self.project_name),
            ));
        }

        let path = format!(
            "{}/{}/{}-data/src/core",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name,
            self.project_name
        );
        let path = Path::new(&path);
        let future_core_module = self.create_module(path);

        let path = format!(
            "{}/{}/{}-data/src/features",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name,
            self.project_name
        );
        let path = Path::new(&path);
        let future_features_module = self.create_module(path);

        let path = format!(
            "{}/{}/{}-data/src/lib.rs",
            self.project_path
                .to_str()
                .expect(&format!("Error parsing path: {:?}", self.project_path)),
            self.project_name,
            self.project_name
        );
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
