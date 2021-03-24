use async_std::fs::DirBuilder;
use async_trait::async_trait;
use std::io;

#[async_trait]
pub trait Datasource {
    async fn create_project_directory(&self, project_name: &str) -> io::Result<()>;
}

pub struct NewDatasource {}

impl NewDatasource {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Datasource for NewDatasource {
    async fn create_project_directory(&self, project_name: &str) -> io::Result<()> {
        DirBuilder::new().create(project_name).await
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        assert_eq!(1, 1)
    }
}
