use async_trait::async_trait;
use std::io;

use project_data::features::new::datasource::{Datasource, NewDatasource};

#[async_trait]
pub trait Repository {
    async fn create_project_directory(&self, project_name: &str) -> io::Result<()>;
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
    async fn create_project_directory(&self, project_name: &str) -> io::Result<()> {
        self.datasource.create_project_directory(project_name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDatasource {}

    #[async_trait]
    impl Datasource for MockDatasource {
        async fn create_project_directory(&self, _project_name: &str) -> io::Result<()> {
            Ok(())
        }
    }

    #[async_std::test]
    async fn test_create_project_directory() {
        let datasource = Box::new(MockDatasource {});
        let repository = NewRepository { datasource };

        let expect = ();
        let got = repository.create_project_directory("test").await.unwrap();

        assert_eq!(expect, got)
    }
}
