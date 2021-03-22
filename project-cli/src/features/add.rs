use futures::executor::block_on;
use project_domain::features::add::usecase::{AddUsecase, Usecase};
use structopt::StructOpt;

use crate::core::run::Run;

#[derive(Debug, PartialEq, StructOpt)]
pub struct Add {
    #[structopt(subcommand)]
    resource: Resource,
}

impl Run for Add {
    fn run(&self) {
        match &self.resource {
            Resource::Feature(feature) => {
                let usecase = AddUsecase::new();
                println!("{}", block_on(usecase.add_feature(&feature.name)))
            }
        }
    }
}

#[derive(Debug, PartialEq, StructOpt)]
enum Resource {
    Feature(Feature),
}

#[derive(Debug, PartialEq, StructOpt)]
struct Feature {
    name: String,
}
