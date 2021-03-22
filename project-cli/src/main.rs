use crate::core::run::Run;

use structopt::StructOpt;

mod core;
mod features;

#[derive(Debug, PartialEq, StructOpt)]
struct Project {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, PartialEq, StructOpt)]
enum Cmd {
    Add(features::add::Add),
}

fn main() {
    let project = Project::from_args();

    match project.cmd {
        Cmd::Add(add) => add.run(),
    }
}
