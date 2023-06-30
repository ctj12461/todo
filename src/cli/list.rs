use std::error::Error;
use std::sync::Arc;

use chrono::{NaiveDateTime, ParseResult};
use clap::{Args, ValueEnum};
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Row, Table};

use crate::domain::entity::{Item, TagSet};
use crate::domain::usecase::select::{self, Request, Response};
use crate::repository::item::Pool;
use crate::repository::Repository;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Group {
    Planned,
    Finished,
    Canceled,
}

#[derive(Args)]
pub struct ListArgs {
    #[arg(short, long, value_enum, default_value_t = Group::Planned)]
    group: Group,
    #[arg(short, long = "tag")]
    tags: Vec<String>,
    #[arg(short, long, value_parser = parse_datetime)]
    before: Option<NaiveDateTime>,
    #[arg(short, long, value_parser = parse_datetime)]
    after: Option<NaiveDateTime>,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn parse_datetime(value: &str) -> ParseResult<NaiveDateTime> {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
}

pub fn run(repo: Arc<Repository>, args: ListArgs) -> Result<(), Box<dyn Error>> {
    let group = args.group;
    let verbose = args.verbose;

    let request = Request {
        tags: args.tags.into_iter().collect(),
        before: args.before,
        after: args.after,
    };

    let func = |pool: &mut dyn Pool| select::execute(pool, request);

    let response = match group {
        Group::Planned => repo.apply_planned(func),
        Group::Finished => repo.apply_finished(func),
        Group::Canceled => repo.apply_canceled(func),
    };

    match response {
        Ok(Response { items }) => {
            println!("{}", build_table(items, verbose));
            Ok(())
        }
        Err(err) => {
            println!("{err}");
            Err(Box::new(err))
        }
    }
}

fn build_table(items: Vec<Item>, verbose: bool) -> Table {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);

    if verbose {
        table.set_header(vec![
            Cell::new("ID").add_attribute(Attribute::Bold),
            Cell::new("Summary").add_attribute(Attribute::Bold),
            Cell::new("Content").add_attribute(Attribute::Bold),
            Cell::new("Deadline").add_attribute(Attribute::Bold),
            Cell::new("Tags").add_attribute(Attribute::Bold),
            Cell::new("Priority").add_attribute(Attribute::Bold),
        ]);

        for item in items {
            let mut row = Row::new();
            row.add_cell(item.id().into());
            row.add_cell(item.summary().into());
            row.add_cell(item.content().into());
            row.add_cell(item.deadline().into());
            row.add_cell(tags_to_cell(item.tags()));
            row.add_cell(item.priority().value().into());
            table.add_row(row);
        }
    } else {
        table.set_header(vec![
            Cell::new("ID").add_attribute(Attribute::Bold),
            Cell::new("Summary").add_attribute(Attribute::Bold),
            Cell::new("Deadline").add_attribute(Attribute::Bold),
        ]);

        for item in items {
            let mut row = Row::new();
            row.add_cell(item.id().into());
            row.add_cell(item.summary().into());
            row.add_cell(item.deadline().into());
            table.add_row(row);
        }
    }

    table
}

fn tags_to_cell(tags: &TagSet) -> Cell {
    let mut res = tags
        .iter()
        .map(|t| String::from("#") + t.as_str())
        .collect::<Vec<_>>();

    res.sort();
    let res = res.join(" ");

    if !res.is_empty() {
        Cell::new(res)
    } else {
        Cell::new("/").set_alignment(CellAlignment::Center)
    }
}
