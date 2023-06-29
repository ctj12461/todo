use std::error::Error;
use std::sync::Arc;

use clap::Args;

use crate::domain::usecase::transfer::{self, Request};
use crate::repository::Repository;

#[derive(Args)]
pub struct CancelArgs {
    #[arg(short, long)]
    id: u64,
}

pub fn run(repo: Arc<Repository>, args: CancelArgs) -> Result<(), Box<dyn Error>> {
    let id = args.id;
    let request = Request { id };

    let response = repo.apply_planned_canceled_ids(|planned, canceled, ids| {
        transfer::execute(planned, canceled, ids, request)
    });

    match response {
        Ok(()) => {
            println!("Mark {id} as canceled");
            Ok(())
        }
        Err(err) => {
            eprintln!("{err}");
            Err(Box::new(err))
        }
    }
}
