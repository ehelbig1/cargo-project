use async_trait::async_trait;
use std::io;

use project_data::features::new::datasource::{Datasource, NewDatasource};

#[async_trait]
pub trait Repository {
    async fn create_git_repo(&self, project_name: &str) -> io::Result<()>;
    async fn create_gitignore(&self, project_name: &str, content: &[u8]) -> io::Result<()>;
    async fn create_cargo_file(&self, project_name: &str, content: &[u8]) -> io::Result<()>;
}

pub struct NewRepository {
    datasource: Box<dyn Datasource + Send + Sync>,
}

impl NewRepository {
    pub fn new() -> Self {
        let datasource = Box::new(NewDatasource::new());
        Self { datasource }
    }
}

#[async_trait]
impl Repository for NewRepository {
    async fn create_git_repo(&self, project_name: &str) -> io::Result<()> {
        self.datasource.create_git_repo(project_name).await
    }

    async fn create_gitignore(&self, project_name: &str, content: &[u8]) -> io::Result<()> {
        self.datasource
            .create_gitignore(project_name, content)
            .await
    }

    async fn create_cargo_file(&self, project_name: &str, content: &[u8]) -> io::Result<()> {
        self.datasource
            .create_cargo_file(project_name, content)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDatasource {}

    #[async_trait]
    impl Datasource for MockDatasource {
        async fn create_git_repo(&self, _project_name: &str) -> io::Result<()> {
            Ok(())
        }

        async fn create_gitignore(&self, _project_name: &str, _content: &[u8]) -> io::Result<()> {
            Ok(())
        }

        async fn create_cargo_file(&self, _project_name: &str, _content: &[u8]) -> io::Result<()> {
            Ok(())
        }
    }
    #[async_std::test]
    async fn test_create_git_repo() {
        let datasource = Box::new(MockDatasource {});
        let repository = NewRepository { datasource };

        let expect = ();
        let got = repository.create_git_repo("test").await.unwrap();

        assert_eq!(expect, got)
    }

    #[async_std::test]
    async fn test_create_gitignore() {
        let datasource = Box::new(MockDatasource {});
        let repository = NewRepository { datasource };

        let expect = ();
        let got = repository.create_gitignore("test", b"test").await.unwrap();

        assert_eq!(expect, got)
    }

    #[async_std::test]
    async fn test_create_cargo_file() {
        let datasource = Box::new(MockDatasource {});
        let repository = NewRepository { datasource };

        let expect = ();
        let got = repository.create_cargo_file("test", b"test").await.unwrap();

        assert_eq!(expect, got)
    }
}
