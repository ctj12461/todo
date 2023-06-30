use std::error::Error;
use std::sync::Arc;

use clap::Args;

use crate::domain::usecase::remove_tag::{self, Request};
use crate::repository::Repository;

#[derive(Args)]
pub struct RemoveTagArgs {
    #[arg(short, long)]
    id: u64,
    #[arg(short, long = "tag")]
    tags: Vec<String>,
}

pub fn run(repo: Arc<Repository>, args: RemoveTagArgs) -> Result<(), Box<dyn Error>> {
    let request = Request {
        id: args.id,
        tags: args.tags.into_iter().collect(),
    };

    let response = repo.apply_planned(|planned| remove_tag::execute(planned, request));

    match response {
        Ok(()) => Ok(()),
        Err(err) => {
            eprintln!("{err}");
            Err(Box::new(err))
        }
    }
}
