use std::error::Error;
use std::sync::Arc;

use clap::Args;

use crate::domain::usecase::transfer::{self, Request};
use crate::repository::Repository;

#[derive(Args)]
pub struct FinishArgs {
    #[arg(short, long)]
    id: u64,
}

pub fn run(repo: Arc<Repository>, args: FinishArgs) -> Result<(), Box<dyn Error>> {
    let id = args.id;
    let request = Request { id };

    let response = repo.apply_planned_finished_ids(|planned, finished, ids| {
        transfer::execute(planned, finished, ids, request)
    });

    match response {
        Ok(()) => {
            println!("Mark {id} as finished");
            Ok(())
        }
        Err(err) => {
            eprintln!("{err}");
            Err(Box::new(err))
        }
    }
}
