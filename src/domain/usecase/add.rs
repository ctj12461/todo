use std::sync::Arc;

use chrono::prelude::*;
use snafu::prelude::*;

use crate::domain::entity::{Item, Priority, TagSet};
use crate::repository::{AddError, Repository};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub title: String,
    pub description: String,
    pub deadline: NaiveDateTime,
    pub tags: TagSet,
    pub priority: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub id: u64,
}

#[derive(Debug, PartialEq, Eq, Snafu)]
pub enum AddItemError {
    #[snafu(display("`title` may not be empty and `priority` should be in [-3, 3]"))]
    Invalid,
    #[snafu(display("Two same items may not exist"))]
    Conflict,
    #[snafu(display("{message}"))]
    Other { message: String },
}

pub fn execute(repo: Arc<dyn Repository>, request: Request) -> Result<Response, AddItemError> {
    let Request {
        title,
        description,
        deadline,
        tags,
        priority,
    } = request;
    ensure!(!title.is_empty(), InvalidSnafu);

    let priority = match Priority::try_from(priority) {
        Ok(v) => v,
        Err(()) => return Err(AddItemError::Invalid),
    };

    let res = repo.add(Item::new(
        title.as_str(),
        description.as_str(),
        deadline,
        tags,
        priority,
    ));

    match res {
        Ok(id) => Ok(Response { id }),
        Err(AddError::Conflict) => Err(AddItemError::Conflict),
        Err(AddError::Other { message }) => Err(AddItemError::Other { message }),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::repository::MemoryRepositry;

    use super::*;

    #[test]
    fn it_should_return_an_id_when_creating_item_succeeded() {
        let item = Item::new_test();
        let id = item.id();

        let request = Request {
            title: item.title().to_owned(),
            description: item.description().to_owned(),
            deadline: *item.deadline(),
            tags: item.tags().clone(),
            priority: item.priority().value(),
        };

        let repo: Arc<dyn Repository> = Arc::new(MemoryRepositry::new());
        let res = execute(repo, request);
        assert_eq!(res, Ok(Response { id }));
    }

    #[test]
    fn it_should_return_invalid_error_when_title_is_empty() {
        let request = Request {
            title: String::new(),
            description: String::from("This is description."),
            deadline: get_deadline(),
            tags: HashSet::new(),
            priority: 0i32,
        };

        let repo: Arc<dyn Repository> = Arc::new(MemoryRepositry::new());
        let res = execute(repo, request);
        assert_eq!(res, Err(AddItemError::Invalid));
    }

    #[test]
    fn it_should_return_invalid_error_when_priority_is_out_of_bound() {
        let request = Request {
            title: String::from("Test"),
            description: String::from("This is description."),
            deadline: get_deadline(),
            tags: HashSet::new(),
            priority: 10i32,
        };

        let repo: Arc<dyn Repository> = Arc::new(MemoryRepositry::new());
        let res = execute(repo, request);
        assert_eq!(res, Err(AddItemError::Invalid));
    }

    #[test]
    fn it_should_return_conflict_error_when_adding_two_same_items() {
        let request = Request {
            title: String::from("Test"),
            description: String::from("This is description."),
            deadline: get_deadline(),
            tags: HashSet::new(),
            priority: 0i32,
        };

        let repo: Arc<dyn Repository> = Arc::new(MemoryRepositry::new());
        let _ = execute(Arc::clone(&repo), request.clone());
        let res = execute(repo, request);
        assert_eq!(res, Err(AddItemError::Conflict));
    }

    #[inline]
    fn get_deadline() -> NaiveDateTime {
        NaiveDateTime::parse_from_str("2023-06-17 23:20:00", "%Y-%m-%d %H:%M:%S").unwrap()
    }
}
