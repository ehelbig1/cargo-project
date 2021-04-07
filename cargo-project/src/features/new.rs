use std::env;
use std::path::PathBuf;

use futures::executor::block_on;
use structopt::StructOpt;

use crate::core::run::Run;
use cargo_project_domain::features::new::usecase::{NewUsecase, Usecase};

#[derive(Debug, PartialEq, StructOpt)]
pub struct New {
    name: String,

    #[structopt(parse(from_os_str), short, long)]
    path: Option<PathBuf>,
}

impl Run for New {
    fn run(&self) {
        match &self.path {
            Some(path) => {
                let usecase = NewUsecase::new(&self.name, &path);
                println!("{}", block_on(usecase.create_project()));
            }
            None => {
                let current_dir =
                    env::current_dir().expect("Error reading current directory from environment");

                let usecase = NewUsecase::new(&self.name, &current_dir);
                println!("{}", block_on(usecase.create_project()));
            }
        }
    }
}
