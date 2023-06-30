pub mod add;
pub mod add_tag;
pub mod cancel;
pub mod clean;
pub mod finish;
pub mod list;
pub mod remove_tag;
pub mod set_priority;

use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

use clap::{Parser, Subcommand};

use crate::repository::Repository;

use add::AddArgs;
use add_tag::AddTagArgs;
use cancel::CancelArgs;
use finish::FinishArgs;
use list::ListArgs;
use remove_tag::RemoveTagArgs;
use set_priority::SetPriorityArgs;

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
    Finish(FinishArgs),
    Cancel(CancelArgs),
    Clean,
    List(ListArgs),
    AddTag(AddTagArgs),
    RemoveTag(RemoveTagArgs),
    SetPriority(SetPriorityArgs),
}

pub fn run(repo: Arc<Repository>, command: Command) -> Result<(), Box<dyn Error>> {
    match command {
        Command::Add(args) => add::run(repo, args),
        Command::Finish(args) => finish::run(repo, args),
        Command::Cancel(args) => cancel::run(repo, args),
        Command::Clean => clean::run(repo),
        Command::List(args) => list::run(repo, args),
        Command::AddTag(args) => add_tag::run(repo, args),
        Command::RemoveTag(args) => remove_tag::run(repo, args),
        Command::SetPriority(args) => set_priority::run(repo, args),
    }
}
