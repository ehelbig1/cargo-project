use async_trait::async_trait;
use project_data::features::add::datasource::{AddDatasource, Datasource};

use super::entities::{UpdateError, UpdateSuccess};

#[async_trait]
pub trait Repository {
    async fn update_cli(&self, name: &str) -> Result<UpdateSuccess, UpdateError>;
    async fn update_domain(&self, name: &str) -> Result<UpdateSuccess, UpdateError>;
    async fn update_data(&self, name: &str) -> Result<UpdateSuccess, UpdateError>;
}

pub struct AddRepository {
    datasource: Box<dyn Datasource + Send + Sync>,
}

impl AddRepository {
    pub fn new() -> Self {
        let datasource = Box::new(AddDatasource::new());
        Self { datasource }
    }
}

#[async_trait]
impl Repository for AddRepository {
    async fn update_cli(&self, name: &str) -> Result<UpdateSuccess, UpdateError> {
        let result = self.datasource.update_cli(name).await;

        match result {
            Ok(model) => return Ok(UpdateSuccess::from_model(model)),
            Err(error) => return Err(UpdateError::from_err(error)),
        }
    }

    async fn update_domain(&self, name: &str) -> Result<UpdateSuccess, UpdateError> {
        let result = self.datasource.update_domain(name).await;

        match result {
            Ok(model) => return Ok(UpdateSuccess::from_model(model)),
            Err(error) => return Err(UpdateError::from_err(error)),
        }
    }

    async fn update_data(&self, name: &str) -> Result<UpdateSuccess, UpdateError> {
        let result = self.datasource.update_data(name).await;

        match result {
            Ok(model) => return Ok(UpdateSuccess::from_model(model)),
            Err(error) => return Err(UpdateError::from_err(error)),
        }
    }
}
