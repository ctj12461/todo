use std::sync::Arc;

use snafu::prelude::*;

use crate::domain::entity::TagSet;
use crate::repository::{AddTagError as RepositoryError, Repository};

pub struct Request {
    pub id: u64,
    pub tags: TagSet,
}

#[derive(Debug, PartialEq, Eq, Snafu)]
pub enum AddTagError {
    #[snafu(display("No item with given id is found"))]
    NotFound,
    #[snafu(display("Some tags have already existed"))]
    Conflict,
    #[snafu(display("{message}"))]
    Other { message: String },
}

pub fn execute(repo: Arc<dyn Repository>, request: Request) -> Result<(), AddTagError> {
    match repo.add_tag(request.id, request.tags) {
        Ok(()) => Ok(()),
        Err(RepositoryError::Conflict) => Err(AddTagError::Conflict),
        Err(RepositoryError::NotFound) => Err(AddTagError::NotFound),
        Err(RepositoryError::Other { message }) => Err(AddTagError::Other { message }),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::domain::entity::Item;
    use crate::repository::MemoryRepositry;

    use super::*;

    #[test]
    fn it_should_return_ok_when_succeeding() {
        let item = Item::new_test();
        let id = item.id();

        let mut map = HashMap::new();
        let _ = map.insert(id, item);
        let repo: Arc<dyn Repository> = Arc::new(MemoryRepositry::from(map));

        let mut tags = TagSet::new();
        tags.insert("a".to_owned());
        tags.insert("b".to_owned());

        let request = Request { id, tags };
        let res = execute(Arc::clone(&repo), request);

        assert_eq!(res, Ok(()));

        if let Ok(item) = repo.get(id) {
            assert!(item.find_tag(&"a".to_owned()));
            assert!(item.find_tag(&"b".to_owned()));
        } else {
            unreachable!()
        }
    }

    #[test]
    fn it_should_return_conflict_error_but_add_the_rest_when_some_tags_exist() {
        let mut item = Item::new_test();
        item.add_tag(String::from("a"));
        let id = item.id();

        let mut map = HashMap::new();
        let _ = map.insert(id, item);
        let repo: Arc<dyn Repository> = Arc::new(MemoryRepositry::from(map));

        let mut tags = TagSet::new();
        tags.insert("a".to_owned());
        tags.insert("b".to_owned());

        let request = Request { id, tags };
        let res = execute(Arc::clone(&repo), request);

        assert_eq!(res, Err(AddTagError::Conflict));

        if let Ok(item) = repo.get(id) {
            assert!(item.find_tag(&"a".to_owned()));
            assert!(item.find_tag(&"b".to_owned()));
        } else {
            unreachable!()
        }
    }

    #[test]
    fn it_should_return_not_found_error_when_target_is_not_found() {
        let repo: Arc<dyn Repository> = Arc::new(MemoryRepositry::new());

        let request = Request {
            id: 0,
            tags: TagSet::new(),
        };

        let res = execute(Arc::clone(&repo), request);
        assert_eq!(res, Err(AddTagError::NotFound));
    }
}
