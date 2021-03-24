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
        self.repository
            .create_git_repo(project_name)
            .await
            .expect(&format!("Error creating git repo for: {}", project_name));

        let future_gitignore = self.repository.create_gitignore(project_name, b"/target");

        try_join!(future_gitignore).expect(&format!("Error creating project: {}", project_name));

        format!("Successfully created project: {}", project_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRepository {}

    #[async_trait]
    impl Repository for MockRepository {
        async fn create_git_repo(&self, _project_name: &str) -> std::io::Result<()> {
            Ok(())
        }

        async fn create_gitignore(
            &self,
            _project_name: &str,
            _content: &[u8],
        ) -> std::io::Result<()> {
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
