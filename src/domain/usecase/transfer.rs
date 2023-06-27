use crate::domain::usecase::add::{self, Request as AddRequest};
use crate::domain::usecase::remove::{self, RemoveItemError, Request as RemoveRequest};
use crate::domain::usecase::remove_id::{self, Request as RemoveIdRequest};
use crate::repository::id::Pool as IdPool;
use crate::repository::item::Pool as ItemPool;

pub type Request = RemoveRequest;
pub type TransferError = RemoveItemError;

pub fn execute(
    source: &mut dyn ItemPool,
    destination: &mut dyn ItemPool,
    ids: &mut dyn IdPool,
    request: Request,
) -> Result<(), TransferError> {
    let id = request.id;
    let request = RemoveRequest { id };
    let item = remove::execute(source, request)?;

    let request = RemoveIdRequest { id };
    let _ = remove_id::execute(ids, request);

    let request = AddRequest {
        title: item.title,
        description: item.description,
        deadline: item.deadline,
        tags: item.tags,
        priority: item.priority.value(),
    };

    let _ = add::execute(destination, request);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::domain::entity::Item;
    use crate::repository::id::{Trie, TriePool};
    use crate::repository::item::{GetError, MemoryPool};

    use super::*;

    #[test]
    fn it_should_return_ok_when_succeeded() {
        let item = Item::new_test();
        let id = item.id();

        let mut map = HashMap::new();
        let _ = map.insert(id, item);
        let mut source: Box<dyn ItemPool> = Box::new(MemoryPool::from(map));
        let mut destination: Box<dyn ItemPool> = Box::new(MemoryPool::new());

        let mut trie = Trie::new();
        trie.insert(id);
        let mut ids: Box<dyn IdPool> = Box::new(TriePool::from(trie));

        let request = Request { id };
        let res = execute(source.as_mut(), destination.as_mut(), ids.as_mut(), request);

        assert_eq!(res, Ok(()));
        assert!(matches!(source.get(id), Err(GetError::NotFound)));
        assert!(matches!(destination.get(id), Ok(_)));
        assert!(!ids.remove(id));
    }

    #[test]
    fn it_should_return_not_found_error_when_the_target_does_not_exist() {
        let mut source: Box<dyn ItemPool> = Box::new(MemoryPool::new());
        let mut destination: Box<dyn ItemPool> = Box::new(MemoryPool::new());
        let mut ids: Box<dyn IdPool> = Box::new(TriePool::new());
        let request = Request { id: 0 };
        let res = execute(source.as_mut(), destination.as_mut(), ids.as_mut(), request);
        assert_eq!(res, Err(TransferError::NotFound));
    }
}
