use chrono::NaiveDateTime;
use snafu::prelude::*;

use crate::domain::entity::{Priority, TagSet};
use crate::repository::item::{Pool, RemoveError};

pub struct Request {
    pub id: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub deadline: NaiveDateTime,
    pub tags: TagSet,
    pub priority: Priority,
}

#[derive(Debug, PartialEq, Eq, Snafu)]
pub enum RemoveItemError {
    #[snafu(display("Target isn't found"))]
    NotFound,
}

pub fn execute(pool: &mut dyn Pool, request: Request) -> Result<Response, RemoveItemError> {
    match pool.remove(request.id) {
        Ok(item) => Ok(Response {
            id: item.id(),
            title: item.title().to_owned(),
            description: item.description().to_owned(),
            deadline: *item.deadline(),
            tags: item.tags().clone(),
            priority: item.priority().clone(),
        }),
        Err(RemoveError::NotFound) => Err(RemoveItemError::NotFound),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::domain::entity::Item;
    use crate::repository::item::MemoryPool;

    use super::*;

    #[test]
    fn it_should_return_the_corresponding_item_when_removing_with_id() {
        let item = Item::new_test();
        let id = item.id();

        let mut map = HashMap::new();
        let _ = map.insert(id, item.clone());
        let mut pool: Box<dyn Pool> = Box::new(MemoryPool::from(map));

        let request = Request { id };
        let res = execute(pool.as_mut(), request);

        assert_eq!(
            res,
            Ok(Response {
                id,
                title: item.title().to_owned(),
                description: item.description().to_owned(),
                deadline: *item.deadline(),
                tags: item.tags().clone(),
                priority: item.priority().clone(),
            })
        );
    }

    #[test]
    fn it_should_return_not_found_error_when_the_target_does_not_exist() {
        let mut pool: Box<dyn Pool> = Box::new(MemoryPool::new());
        let request = Request { id: 0u64 };
        let res = execute(pool.as_mut(), request);
        assert_eq!(res, Err(RemoveItemError::NotFound));
    }
}
