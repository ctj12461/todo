use std::error::Error;
use std::sync::Arc;

use clap::Args;

use crate::domain::entity::Priority;
use crate::domain::usecase::set_priority::{self, Request};
use crate::repository::Repository;

#[derive(Args)]
pub struct SetPriorityArgs {
    #[arg(short, long)]
    id: u64,
    #[arg(short, long, default_value_t = 0.try_into().unwrap(), value_parser = parse_priority)]
    priority: Priority,
}

fn parse_priority(value: &str) -> Result<Priority, String> {
    value
        .parse::<i32>()
        .map_err(|err| err.to_string())?
        .try_into()
        .map_err(|_| String::from("`priority` should be in [-3, 3]"))
}

pub fn run(repo: Arc<Repository>, args: SetPriorityArgs) -> Result<(), Box<dyn Error>> {
    let request = Request {
        id: args.id,
        priority: args.priority.value(),
    };

    let response = repo.apply_planned(|planned| set_priority::execute(planned, request));

    match response {
        Ok(()) => Ok(()),
        Err(err) => {
            eprintln!("{err}");
            Err(Box::new(err))
        }
    }
}
