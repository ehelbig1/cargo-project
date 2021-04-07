use async_trait::async_trait;
use std::io;
use std::path::Path;

use cargo_project_data::features::new::datasource::{Datasource, NewDatasource};

#[async_trait]
pub trait Repository {
    async fn create_git_repo(&self) -> io::Result<()>;
    async fn create_gitignore(&self, content: &[u8]) -> io::Result<()>;
    async fn create_cargo_file(&self, content: &[u8]) -> io::Result<()>;
    async fn create_presentation_layer(&self, main_file_content: &[u8]) -> io::Result<()>;
    async fn create_domain_layer(&self, main_file_content: &[u8]) -> io::Result<()>;
    async fn create_data_layer(&self, main_file_content: &[u8]) -> io::Result<()>;
}

pub struct NewRepository<'a> {
    datasource: Box<dyn Datasource + Send + Sync + 'a>,
}

impl<'a> NewRepository<'a> {
    pub fn new(project_name: &'a str, project_path: &'a Path) -> Self {
        let datasource = Box::new(NewDatasource::new(project_name, project_path));
        Self { datasource }
    }
}

#[async_trait]
impl<'a> Repository for NewRepository<'a> {
    async fn create_git_repo(&self) -> io::Result<()> {
        self.datasource.create_git_repo().await
    }

    async fn create_gitignore(&self, content: &[u8]) -> io::Result<()> {
        self.datasource.create_gitignore(content).await
    }

    async fn create_cargo_file(&self, content: &[u8]) -> io::Result<()> {
        self.datasource.create_cargo_file(content).await
    }

    async fn create_presentation_layer(&self, main_file_content: &[u8]) -> io::Result<()> {
        self.datasource
            .create_presentation_layer(main_file_content)
            .await
    }

    async fn create_domain_layer(&self, main_file_content: &[u8]) -> io::Result<()> {
        self.datasource.create_domain_layer(main_file_content).await
    }

    async fn create_data_layer(&self, main_file_content: &[u8]) -> io::Result<()> {
        self.datasource.create_data_layer(main_file_content).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDatasource {}

    #[async_trait]
    impl Datasource for MockDatasource {
        async fn create_git_repo(&self) -> io::Result<()> {
            Ok(())
        }

        async fn create_gitignore(&self, _content: &[u8]) -> io::Result<()> {
            Ok(())
        }

        async fn create_cargo_file(&self, _content: &[u8]) -> io::Result<()> {
            Ok(())
        }

        async fn create_presentation_layer(&self, _main_file_content: &[u8]) -> io::Result<()> {
            Ok(())
        }

        async fn create_domain_layer(&self, _main_file_content: &[u8]) -> io::Result<()> {
            Ok(())
        }

        async fn create_data_layer(&self, _main_file_content: &[u8]) -> io::Result<()> {
            Ok(())
        }
    }
    #[async_std::test]
    async fn test_create_git_repo() {
        let datasource = Box::new(MockDatasource {});
        let repository = NewRepository { datasource };

        let expect = ();
        let got = repository.create_git_repo().await.unwrap();

        assert_eq!(expect, got)
    }

    #[async_std::test]
    async fn test_create_gitignore() {
        let datasource = Box::new(MockDatasource {});
        let repository = NewRepository { datasource };

        let expect = ();
        let got = repository.create_gitignore(b"test").await.unwrap();

        assert_eq!(expect, got)
    }

    #[async_std::test]
    async fn test_create_cargo_file() {
        let datasource = Box::new(MockDatasource {});
        let repository = NewRepository { datasource };

        let expect = ();
        let got = repository.create_cargo_file(b"test").await.unwrap();

        assert_eq!(expect, got)
    }

    #[async_std::test]
    async fn test_create_presentation() {
        let datasource = Box::new(MockDatasource {});
        let repository = NewRepository { datasource };

        let expect = ();
        let got = repository.create_presentation_layer(b"test").await.unwrap();

        assert_eq!(expect, got)
    }

    #[async_std::test]
    async fn test_create_domain() {
        let datasource = Box::new(MockDatasource {});
        let repository = NewRepository { datasource };

        let expect = ();
        let got = repository.create_domain_layer(b"test").await.unwrap();

        assert_eq!(expect, got)
    }

    #[async_std::test]
    async fn test_create_data() {
        let datasource = Box::new(MockDatasource {});
        let repository = NewRepository { datasource };

        let expect = ();
        let got = repository.create_data_layer(b"test").await.unwrap();

        assert_eq!(expect, got)
    }
}
