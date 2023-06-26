mod trie;

pub use trie::{Trie, TriePool};

pub trait Pool {
    fn add(&mut self, id: u64) -> bool;

    fn remove(&mut self, id: u64) -> bool;

    fn find(&self, pattern: u64) -> Option<Vec<u64>>;
}
