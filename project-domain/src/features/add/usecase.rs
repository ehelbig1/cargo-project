use async_trait::async_trait;
use futures::try_join;

use super::repository::{AddRepository, Repository};

#[async_trait]
pub trait Usecase {
    async fn add_feature(&self, name: &str) -> String;
}

pub struct AddUsecase {
    repository: Box<dyn Repository + Send + Sync>,
}

impl AddUsecase {
    pub fn new() -> Self {
        let repository = Box::new(AddRepository::new());
        Self { repository }
    }
}

#[async_trait]
impl Usecase for AddUsecase {
    async fn add_feature(&self, name: &str) -> String {
        let cli_update_future = self.repository.update_cli(name);
        let domain_update_future = self.repository.update_domain(name);
        let data_update_future = self.repository.update_data(name);

        // let (cli_update_result, domain_update_response, data_update_response) =
        let result = try_join!(cli_update_future, domain_update_future, data_update_future);

        match result {
            Ok(_) => return format!("Successfully added feature: {}", name),
            Err(err) => return format!("Failed to add feature: {} - {:?}", name, err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::entities::{UpdateError, UpdateSuccess};

    use async_trait::async_trait;

    struct MockRepository {}

    #[async_trait]
    impl Repository for MockRepository {
        async fn update_cli(&self, _name: &str) -> Result<UpdateSuccess, UpdateError> {
            Ok(UpdateSuccess::new(String::from("Successfully updated cli")))
        }

        async fn update_domain(&self, _name: &str) -> Result<UpdateSuccess, UpdateError> {
            Ok(UpdateSuccess::new(String::from(
                "Successfully updated domain",
            )))
        }

        async fn update_data(&self, _name: &str) -> Result<UpdateSuccess, UpdateError> {
            Ok(UpdateSuccess::new(String::from(
                "Successfully updated data",
            )))
        }
    }

    #[async_std::test]
    async fn test_add_feature() {
        let repository = Box::new(MockRepository {});
        let usecase = AddUsecase { repository };

        let expect = String::from("Successfully added feature: test");
        let got = usecase.add_feature("test").await;

        assert_eq!(expect, got)
    }
}
