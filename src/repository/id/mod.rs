mod trie;

pub use trie::{Trie, TriePool};

pub trait Pool {
    fn add(&mut self, id: u64) -> bool;

    fn remove(&mut self, id: u64) -> bool;
}
