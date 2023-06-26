use snafu::prelude::*;

use crate::repository::id::Pool;

pub struct Request {
    pub pattern: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    pub id: u64,
}

#[derive(Debug, PartialEq, Eq, Snafu)]
pub enum CompleteIdError {
    #[snafu(display("No ID starts with the given pattern"))]
    NotFound,
    #[snafu(display("The given pattern is ambiguous"))]
    Ambiguous,
}

pub fn execute(pool: &dyn Pool, request: Request) -> Result<Response, CompleteIdError> {
    let res = pool.find(request.pattern);

    if let Some(res) = res {
        if res.len() == 1 {
            Ok(Response {
                id: *res.first().unwrap(),
            })
        } else {
            Err(CompleteIdError::Ambiguous)
        }
    } else {
        Err(CompleteIdError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::id::{Trie, TriePool};

    use super::*;

    #[test]
    fn it_should_return_an_unique_id_when_finding_id_starting_with_a_prefix() {
        let mut trie = Trie::new();
        trie.insert(111411222);
        trie.insert(1114333);
        let pool: Box<dyn Pool> = Box::new(TriePool::from(trie));

        let request = Request { pattern: 11141 };
        let res = execute(pool.as_ref(), request);
        assert_eq!(res, Ok(Response { id: 111411222 }));
    }

    #[test]
    fn it_should_return_a_not_found_error_when_no_id_matches_the_pattern() {
        let mut trie = Trie::new();
        trie.insert(111411222);
        trie.insert(1114333);
        let pool: Box<dyn Pool> = Box::new(TriePool::from(trie));

        let request = Request { pattern: 11151 };
        let res = execute(pool.as_ref(), request);
        assert_eq!(res, Err(CompleteIdError::NotFound));
    }

    #[test]
    fn it_should_return_an_ambiguous_error_when_many_ids_match_the_pattern() {
        let mut trie = Trie::new();
        trie.insert(111411222);
        trie.insert(1114333);
        let pool: Box<dyn Pool> = Box::new(TriePool::from(trie));

        let request = Request { pattern: 111 };
        let res = execute(pool.as_ref(), request);
        assert_eq!(res, Err(CompleteIdError::Ambiguous));
    }
}
