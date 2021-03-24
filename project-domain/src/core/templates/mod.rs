use std::error::Error;

pub mod cargo;
pub mod cli;
pub mod datasource;
pub mod entities;
pub mod mod_file;
pub mod models;
pub mod repository;
pub mod usecase;

pub trait Template {
    fn render(&self) -> Result<String, Box<dyn Error>>;
}
