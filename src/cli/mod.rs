pub mod add;
pub mod cancel;
pub mod clean;
pub mod finish;

use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

use clap::{Parser, Subcommand};

use crate::repository::Repository;

use add::AddArgs;
use cancel::CancelArgs;
use finish::FinishArgs;

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
    Cancel(CancelArgs),
    Clean,
    Finish(FinishArgs),
}

pub fn run(repo: Arc<Repository>, command: Command) -> Result<(), Box<dyn Error>> {
    match command {
        Command::Add(args) => add::run(repo, args),
        Command::Cancel(args) => cancel::run(repo, args),
        Command::Clean => clean::run(repo),
        Command::Finish(args) => finish::run(repo, args),
    }
}
