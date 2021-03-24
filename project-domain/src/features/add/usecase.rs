use async_trait::async_trait;
use futures::try_join;
use titlecase::titlecase;

use super::repository::{AddRepository, Repository};

use crate::core::templates::cli::CliTemplate;
use crate::core::templates::datasource::DatasourceTemplate;
use crate::core::templates::entities::EntitiesTemplate;
use crate::core::templates::mod_file::ModFileTemplate;
use crate::core::templates::models::ModelsTemplate;
use crate::core::templates::repository::RepositoryTemplate;
use crate::core::templates::usecase::UsecaseTemplate;
use crate::core::templates::Template;

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
        const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

        let mut project = PKG_NAME.split("-");
        let project = project
            .next()
            .expect(&format!("Error parsing cargo package name: {}", PKG_NAME));
        let name_title_case = titlecase(name);

        let cli_file_content = CliTemplate::new(name, &name_title_case).render().unwrap();
        let cli_parent_mod_file_content = ModFileTemplate::new(vec![name]).render().unwrap();

        let entities_file_content = EntitiesTemplate::new(name).render().unwrap();
        let domain_mod_file_content =
            ModFileTemplate::new(vec!["entities", "repository", "usecase"])
                .render()
                .unwrap();
        let domain_parent_mod_file_content = ModFileTemplate::new(vec![name]).render().unwrap();
        let repository_file_content = RepositoryTemplate::new(name, &name_title_case, project)
            .render()
            .unwrap();
        let usecase_file_content = UsecaseTemplate::new(&name_title_case).render().unwrap();

        let data_mod_file_content = ModFileTemplate::new(vec!["datasource", "models"])
            .render()
            .unwrap();
        let data_parent_mod_file_content = ModFileTemplate::new(vec![name]).render().unwrap();
        let datasource_file_content = DatasourceTemplate::new(&name_title_case).render().unwrap();
        let models_file_content = ModelsTemplate::new(name).render().unwrap();

        let cli_update_future = self.repository.update_cli(
            project,
            name,
            cli_file_content.as_bytes(),
            cli_parent_mod_file_content.as_bytes(),
        );
        let domain_update_future = self.repository.update_domain(
            project,
            name,
            entities_file_content.as_bytes(),
            domain_mod_file_content.as_bytes(),
            repository_file_content.as_bytes(),
            usecase_file_content.as_bytes(),
            domain_parent_mod_file_content.as_bytes(),
        );
        let data_update_future = self.repository.update_data(
            project,
            name,
            datasource_file_content.as_bytes(),
            data_mod_file_content.as_bytes(),
            models_file_content.as_bytes(),
            data_parent_mod_file_content.as_bytes(),
        );

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

    use async_trait::async_trait;
    use std::io;

    struct MockRepository {}

    #[async_trait]
    impl Repository for MockRepository {
        async fn update_cli(
            &self,
            _project: &str,
            _name: &str,
            _cli_file_content: &[u8],
            _parent_mod_file_content: &[u8],
        ) -> io::Result<()> {
            Ok(())
        }

        async fn update_domain(
            &self,
            _project: &str,
            _name: &str,
            _entities_file_content: &[u8],
            _mod_file_content: &[u8],
            _repository_file_content: &[u8],
            _usecase_file_content: &[u8],
            _parent_mod_file_content: &[u8],
        ) -> io::Result<()> {
            Ok(())
        }

        async fn update_data(
            &self,
            _project: &str,
            _name: &str,
            _datasource_file_content: &[u8],
            _mod_file_content: &[u8],
            _models_file_content: &[u8],
            _parent_mod_file_content: &[u8],
        ) -> io::Result<()> {
            Ok(())
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
