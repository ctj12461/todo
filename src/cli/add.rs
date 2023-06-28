use std::error::Error;
use std::sync::Arc;

use chrono::{NaiveDateTime, ParseResult};
use clap::Args;

use crate::domain::entity::Priority;
use crate::domain::usecase::plan::{self, Request, Response};
use crate::repository::Repository;

#[derive(Args)]
pub struct AddArgs {
    #[arg(short = 's', long)]
    title: String,
    #[arg(short = 'c', long, default_value_t = String::new())]
    description: String,
    #[arg(short = 'd', long, value_parser = parse_datetime)]
    deadline: NaiveDateTime,
    #[arg(short = 't', long = "tag")]
    tags: Vec<String>,
    #[arg(short = 'p', long, default_value_t = 0.try_into().unwrap(), value_parser = parse_priority)]
    priority: Priority,
}

fn parse_datetime(value: &str) -> ParseResult<NaiveDateTime> {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
}

fn parse_priority(value: &str) -> Result<Priority, String> {
    value
        .parse::<i32>()
        .map_err(|err| err.to_string())?
        .try_into()
        .map_err(|_| String::from("`priority` should be in [-3, 3]"))
}

pub fn run(repo: Arc<Repository>, args: AddArgs) -> Result<(), Box<dyn Error>> {
    let request = Request {
        title: args.title,
        description: args.description,
        deadline: args.deadline,
        tags: args.tags.into_iter().collect(),
        priority: args.priority.value(),
    };

    let response = repo.apply_planned_ids(|planned, ids| plan::execute(planned, ids, request));

    match response {
        Ok(Response { id }) => {
            println!("New item: {id}");
            Ok(())
        }
        Err(err) => {
            eprintln!("{err}");
            Err(Box::new(err))
        }
    }
}
