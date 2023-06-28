use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;

use todo::cli::{self, Arg};
use todo::repository::id::TriePool;
use todo::repository::item::LocalPool;
use todo::repository::{Data, Repository};

fn main() -> Result<(), Box<dyn Error>> {
    let Arg { storage, command } = Arg::parse();
    let repo = init(storage);

    let command = match command {
        Some(cmd) => cmd,
        None => return Ok(()),
    };

    cli::run(repo, command)
}

fn init(storage: Option<PathBuf>) -> Arc<Repository> {
    let dir = storage.unwrap_or(default_path());
    let planned_pool_path = dir.join("planned.json");
    let finished_pool_path = dir.join("finished.json");
    let canceled_pool_path = dir.join("canceled.json");

    Arc::new(Repository::new(Data {
        planned: Box::new(LocalPool::open(planned_pool_path).unwrap()),
        finished: Box::new(LocalPool::open(finished_pool_path).unwrap()),
        canceled: Box::new(LocalPool::open(canceled_pool_path).unwrap()),
        ids: Box::new(TriePool::new()),
    }))
}

fn default_path() -> PathBuf {
    "tmp".into()
}
