use async_std::{fs::File, io::prelude::WriteExt, path::Path};
use async_trait::async_trait;
use std::io;
use std::process::Command;

#[async_trait]
pub trait Datasource {
    async fn create_git_repo(&self, project_name: &str) -> io::Result<()>;
    async fn create_gitignore(&self, project_name: &str, content: &[u8]) -> io::Result<()>;
}

pub struct NewDatasource {}

impl NewDatasource {
    pub fn new() -> Self {
        Self {}
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
        } else {
            return Ok(());
        }
    }

    async fn create_gitignore(&self, project_name: &str, content: &[u8]) -> io::Result<()> {
        let path = format!("{}/.gitignore", project_name);
        let path = Path::new(&path);
        let mut file = File::create(path).await?;

        file.write_all(content).await
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        assert_eq!(1, 1)
    }
}
