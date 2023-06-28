pub mod add;

use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

use clap::{Parser, Subcommand};

use crate::repository::Repository;

use add::AddArgs;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Arg {
    #[arg(long)]
    pub storage: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Add(AddArgs),
}

pub fn run(repo: Arc<Repository>, command: Command) -> Result<(), Box<dyn Error>> {
    match command {
        Command::Add(args) => add::run(repo, args),
    }
}
