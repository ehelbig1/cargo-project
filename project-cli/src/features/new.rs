use futures::executor::block_on;
use structopt::StructOpt;

use crate::core::run::Run;
use project_domain::features::new::usecase::{NewUsecase, Usecase};

#[derive(Debug, PartialEq, StructOpt)]
pub struct New {
    name: String,
}

impl Run for New {
    fn run(&self) {
        let usecase = NewUsecase::new();
        println!("{}", block_on(usecase.create_project(&self.name)));
    }
}
