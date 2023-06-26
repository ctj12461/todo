use snafu::prelude::*;

use crate::repository::id::Pool;

pub struct Request {
    pub id: u64,
}

#[derive(Debug, PartialEq, Eq, Snafu)]
pub enum RemoveIdError {
    #[snafu(display("ID doesn't exist"))]
    NotFound,
}

pub fn execute(pool: &mut dyn Pool, request: Request) -> Result<(), RemoveIdError> {
    if pool.remove(request.id) {
        Ok(())
    } else {
        Err(RemoveIdError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::id::{Trie, TriePool};

    use super::*;

    #[test]
    fn it_should_return_ok_when_succeeded() {
        let mut trie = Trie::new();
        trie.insert(123456);
        let mut pool: Box<dyn Pool> = Box::new(TriePool::from(trie));

        let request = Request { id: 123456 };
        assert_eq!(execute(pool.as_mut(), request), Ok(()));
    }

    #[test]
    fn it_should_return_conflict_error_when_id_does_not_exist() {
        let mut trie = Trie::new();
        trie.insert(111111);
        let mut pool: Box<dyn Pool> = Box::new(TriePool::from(trie));

        let request = Request { id: 123456 };
        assert_eq!(
            execute(pool.as_mut(), request),
            Err(RemoveIdError::NotFound)
        );
    }
}
