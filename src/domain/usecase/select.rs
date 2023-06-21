use chrono::NaiveDateTime;
use snafu::prelude::*;

use crate::domain::entity::{Item, TagSet};
use crate::repository::item::{Pool, SelectError};

pub struct Request {
    tags: TagSet,
    before: Option<NaiveDateTime>,
    after: Option<NaiveDateTime>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    items: Vec<Item>,
}

#[derive(Debug, PartialEq, Eq, Snafu)]
pub enum SelectItemError {
    #[snafu(display("Invalid time range [before, after]"))]
    Invalid,
    #[snafu(display("No suitable item is found"))]
    NotFound,
}

pub fn execute(repo: &dyn Pool, request: Request) -> Result<Response, SelectItemError> {
    match repo.select(request.tags, request.before, request.after) {
        Ok(items) => Ok(Response { items }),
        Err(SelectError::Invalid) => Err(SelectItemError::Invalid),
        Err(SelectError::NotFound) => Err(SelectItemError::NotFound),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chrono::NaiveDateTime;

    use crate::repository::item::MemoryPool;

    use super::*;

    #[test]
    fn it_should_return_a_set_of_items_containing_given_tags() {
        let mut m = HashMap::new();
        add(&mut m, new("1", "2023-06-18 3:51:01", 1, &["a"]));
        add(&mut m, new("2", "2023-06-18 3:51:01", 1, &["b"]));
        add(&mut m, new("3", "2023-06-18 3:51:01", 1, &["c"]));
        add(&mut m, new("4", "2023-06-18 3:51:01", 1, &["a", "b"]));
        add(&mut m, new("5", "2023-06-18 3:51:00", 1, &["a", "c"]));
        add(&mut m, new("6", "2023-06-18 3:51:00", 0, &["b", "c"]));
        add(&mut m, new("7", "2023-06-18 3:51:00", 0, &["a", "b", "c"]));
        let repo: Box<dyn Pool> = Box::new(MemoryPool::from(m));

        let tags = ["a", "b"].iter().map(|&t| t.to_owned()).collect();

        let request = Request {
            tags,
            before: None,
            after: None,
        };

        let res = execute(repo.as_ref(), request);

        assert_eq!(
            res,
            Ok(Response {
                items: vec![
                    new("7", "2023-06-18 3:51:00", 0, &["a", "b", "c"]),
                    new("4", "2023-06-18 3:51:01", 1, &["a", "b"])
                ]
            })
        );
    }

    #[test]
    fn it_should_return_all_items_when_the_given_tag_set_is_empty() {
        let mut m = HashMap::new();
        add(&mut m, new("1", "2023-06-18 3:51:00", 1, &["a"]));
        add(&mut m, new("2", "2023-06-18 3:51:01", 2, &["b"]));
        add(&mut m, new("3", "2023-06-18 3:51:00", 2, &[]));

        let repo: Box<dyn Pool> = Box::new(MemoryPool::from(m));

        let request = Request {
            tags: TagSet::new(),
            before: None,
            after: None,
        };

        let res = execute(repo.as_ref(), request);

        assert_eq!(
            res,
            Ok(Response {
                items: vec![
                    new("3", "2023-06-18 3:51:00", 2, &[]),
                    new("1", "2023-06-18 3:51:00", 1, &["a"]),
                    new("2", "2023-06-18 3:51:01", 2, &["b"]),
                ]
            })
        );
    }

    #[test]
    fn it_should_return_not_found_when_no_suitable_item_exists() {
        let mut m = HashMap::new();
        add(&mut m, new("1", "2023-06-18 3:51:00", 1, &["a"]));
        add(&mut m, new("2", "2023-06-18 3:51:01", 2, &["b"]));
        add(&mut m, new("3", "2023-06-18 3:51:00", 2, &["c"]));

        let repo: Box<dyn Pool> = Box::new(MemoryPool::from(m));

        let time = "2023-06-18 3:50:00";

        let request = Request {
            tags: TagSet::new(),
            before: Some(NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S").unwrap()),
            after: None,
        };

        let res = execute(repo.as_ref(), request);
        assert_eq!(res, Err(SelectItemError::NotFound));
    }

    #[test]
    fn it_should_return_not_found_when_no_such_tag_exists() {
        let mut m = HashMap::new();
        add(&mut m, new("1", "2023-06-18 3:51:00", 1, &["a"]));
        add(&mut m, new("2", "2023-06-18 3:51:01", 2, &["b"]));
        add(&mut m, new("3", "2023-06-18 3:51:00", 2, &["c"]));

        let repo: Box<dyn Pool> = Box::new(MemoryPool::from(m));

        let tags = ["d"].iter().map(|&t| t.to_owned()).collect();

        let request = Request {
            tags,
            before: None,
            after: None,
        };

        let res = execute(repo.as_ref(), request);
        assert_eq!(res, Err(SelectItemError::NotFound));
    }

    fn new(title: &str, time: &str, priority: i32, tags: &[&str]) -> Item {
        Item::new(
            title,
            "",
            NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S").unwrap(),
            tags.iter().map(|&t| t.to_owned()).collect(),
            priority.try_into().unwrap(),
        )
    }

    fn add(map: &mut HashMap<u64, Item>, item: Item) {
        let _ = map.insert(item.id(), item);
    }
}
