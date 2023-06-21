use snafu::prelude::*;

use crate::repository::item::{Pool, SetPriorityError as RepositoryError};

pub struct Request {
    pub id: u64,
    pub priority: i32,
}

#[derive(Debug, PartialEq, Eq, Snafu)]
pub enum SetPriorityError {
    #[snafu(display("Priority should be in [-3, 3]"))]
    Invalid,
    #[snafu(display("Target isn't found"))]
    NotFound,
}

pub fn execute(repo: &mut dyn Pool, request: Request) -> Result<(), SetPriorityError> {
    let Request { id, priority } = request;
    let priority = priority.try_into().map_err(|_| SetPriorityError::Invalid)?;

    match repo.set_priority(id, priority) {
        Ok(()) => Ok(()),
        Err(RepositoryError::NotFound) => Err(SetPriorityError::NotFound),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::domain::entity::Item;
    use crate::repository::item::MemoryPool;

    use super::*;

    #[test]
    fn it_should_return_ok_when_succeeded() {
        let item = Item::new_test();
        let id = item.id();

        let mut map = HashMap::new();
        let _ = map.insert(id, item);
        let mut repo: Box<dyn Pool> = Box::new(MemoryPool::from(map));

        let request = Request { id, priority: 3 };
        let res = execute(repo.as_mut(), request);

        assert_eq!(res, Ok(()));

        if let Ok(item) = repo.get(id) {
            assert_eq!(3, item.priority().value());
        } else {
            unreachable!();
        }
    }

    #[test]
    fn it_should_return_not_found_error_when_the_target_does_not_exist() {
        let mut repo: Box<dyn Pool> = Box::new(MemoryPool::new());

        let request = Request {
            id: 0u64,
            priority: Default::default(),
        };

        let res = execute(repo.as_mut(), request);
        assert_eq!(res, Err(SetPriorityError::NotFound));
    }

    #[test]
    fn it_should_return_invalid_when_priority_is_out_of_bound() {
        let item = Item::new_test();
        let id = item.id();

        let mut map = HashMap::new();
        let _ = map.insert(id, item);
        let mut repo: Box<dyn Pool> = Box::new(MemoryPool::from(map));

        let request = Request { id, priority: 10 };
        let res = execute(repo.as_mut(), request);

        assert_eq!(res, Err(SetPriorityError::Invalid));

        if let Ok(item) = repo.get(id) {
            assert_eq!(0, item.priority().value());
        } else {
            unreachable!();
        }
    }
}
