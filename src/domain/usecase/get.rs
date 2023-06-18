use std::sync::Arc;

use chrono::NaiveDateTime;
use snafu::prelude::*;

use crate::domain::entity::{Priority, TagSet};
use crate::repository::{GetError, Repository};

pub struct Request {
    pub id: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Response {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub deadline: NaiveDateTime,
    pub tags: TagSet,
    pub priority: Priority,
}

#[derive(Debug, PartialEq, Eq, Snafu)]
pub enum GetItemError {
    #[snafu(display("Target isn't found"))]
    NotFound,
    #[snafu(display("{message}"))]
    Other { message: String },
}

pub fn execute(repo: Arc<dyn Repository>, request: Request) -> Result<Response, GetItemError> {
    match repo.get(request.id) {
        Ok(item) => Ok(Response {
            id: item.id(),
            title: item.title().to_owned(),
            description: item.description().to_owned(),
            deadline: *item.deadline(),
            tags: item.tags().clone(),
            priority: item.priority().clone(),
        }),
        Err(GetError::NotFound) => Err(GetItemError::NotFound),
        Err(GetError::Other { message }) => Err(GetItemError::Other { message }),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::domain::entity::Item;
    use crate::repository::MemoryRepositry;

    use super::*;

    #[test]
    fn it_should_return_the_corresponding_item_when_getting_item() {
        let item = Item::new_test();
        let id = item.id();

        let mut map = HashMap::new();
        let _ = map.insert(id, item.clone());
        let repo: Arc<dyn Repository> = Arc::new(MemoryRepositry::from(map));

        let request = Request { id };
        let res = execute(Arc::clone(&repo), request);

        let response = Response {
            id,
            title: item.title().to_owned(),
            description: item.description().to_owned(),
            deadline: *item.deadline(),
            tags: item.tags().clone(),
            priority: item.priority().clone(),
        };

        assert_eq!(res, Ok(response.clone()));

        // Do twice
        let request = Request { id };
        let res = execute(Arc::clone(&repo), request);
        assert_eq!(res, Ok(response));
    }

    #[test]
    fn it_should_return_not_found_error_when_the_target_does_not_exist() {
        let repo: Arc<dyn Repository> = Arc::new(MemoryRepositry::new());
        let request = Request { id: 0u64 };
        let res = execute(Arc::clone(&repo), request);
        assert_eq!(res, Err(GetItemError::NotFound));
    }
}
