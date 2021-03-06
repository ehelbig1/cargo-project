use crate::core::run::Run;

use structopt::StructOpt;

mod core;
mod features;

#[derive(Debug, PartialEq, StructOpt)]
struct Project {
    #[structopt(subcommand)]
    cmd: Cmd,

    // Handling this binary being called on it's own: cargo-project new test
    // and being called via cargo:                    cargo project new test
    cargo_project: Option<String>,
}

#[derive(Debug, PartialEq, StructOpt)]
enum Cmd {
    Add(features::add::Add),
    New(features::new::New),
}

fn main() {
    let project = Project::from_args();

    match project.cmd {
        Cmd::Add(add) => add.run(),
        Cmd::New(new) => new.run(),
    }
}
