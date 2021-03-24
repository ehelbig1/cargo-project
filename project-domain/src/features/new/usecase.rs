use async_trait::async_trait;
use futures::try_join;

use super::repository::{NewRepository, Repository};

#[async_trait]
pub trait Usecase {
    async fn create_project(&self, project_name: &str) -> String;
}

pub struct NewUsecase {
    repository: Box<dyn Repository + Send + Sync>,
}

impl NewUsecase {
    pub fn new() -> Self {
        let repository = Box::new(NewRepository::new());
        Self { repository }
    }
}

#[async_trait]
impl Usecase for NewUsecase {
    async fn create_project(&self, project_name: &str) -> String {
        let future_project_directory = self.repository.create_project_directory(project_name);
        let future_git_repo = self.repository.create_git_repo(project_name);

        try_join!(future_project_directory, future_git_repo).expect("Error creating project");

        format!("Successfully created project: {}", project_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRepository {}

    #[async_trait]
    impl Repository for MockRepository {
        async fn create_project_directory(&self, _project_name: &str) -> std::io::Result<()> {
            Ok(())
        }

        async fn create_git_repo(&self, _project_name: &str) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[async_std::test]
    async fn test_create_project() {
        let repository = Box::new(MockRepository {});
        let usecase = NewUsecase { repository };

        let expect = String::from("Successfully created project: test");
        let got = usecase.create_project("test").await;

        assert_eq!(expect, got)
    }
}
