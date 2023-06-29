pub mod local;
pub mod memory;

use crate::domain::entity::{Item, Priority, TagSet};

use chrono::NaiveDateTime;

pub use local::LocalPool;
pub use memory::MemoryPool;

pub trait Pool {
    fn add(&mut self, item: Item) -> Result<u64, AddError>;

    fn remove(&mut self, id: u64) -> Result<Item, RemoveError>;

    fn get(&self, id: u64) -> Result<Item, GetError>;

    fn select(
        &self,
        tags: TagSet,
        before: Option<NaiveDateTime>,
        after: Option<NaiveDateTime>,
    ) -> Result<Vec<Item>, SelectError>;

    fn add_tag(&mut self, id: u64, tags: TagSet) -> Result<(), AddTagError>;

    fn remove_tag(&mut self, id: u64, tags: TagSet) -> Result<(), RemoveTagError>;

    fn set_priority(&mut self, id: u64, priority: Priority) -> Result<(), SetPriorityError>;

    fn clear(&mut self);
}

pub enum AddError {
    Conflict,
}

pub enum RemoveError {
    NotFound,
}

pub enum GetError {
    NotFound,
}

pub enum SelectError {
    Invalid,
    NotFound,
}

pub enum AddTagError {
    NotFound,
    Conflict,
}

pub enum RemoveTagError {
    ItemNotFound,
    TagNotFound,
    Conflict,
}

pub enum SetPriorityError {
    NotFound,
}
