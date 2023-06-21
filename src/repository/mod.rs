pub mod local;
pub mod memory;

use crate::domain::entity::{Item, Priority, TagSet};

use chrono::NaiveDateTime;

pub use local::LocalRepository;
pub use memory::MemoryRepositry;

pub trait Repository: Send + Sync {
    fn add(&self, item: Item) -> Result<u64, AddError>;

    fn remove(&self, id: u64) -> Result<Item, RemoveError>;

    fn get(&self, id: u64) -> Result<Item, GetError>;

    fn select(
        &self,
        tags: TagSet,
        before: Option<NaiveDateTime>,
        after: Option<NaiveDateTime>,
    ) -> Result<Vec<Item>, SelectError>;

    fn add_tag(&self, id: u64, tags: TagSet) -> Result<(), AddTagError>;

    fn remove_tag(&self, id: u64, tags: TagSet) -> Result<(), RemoveTagError>;

    fn set_priority(&self, id: u64, priority: Priority) -> Result<(), SetPriorityError>;
}

pub enum AddError {
    Conflict,
    Other { message: String },
}

pub enum RemoveError {
    NotFound,
    Other { message: String },
}

pub enum GetError {
    NotFound,
    Other { message: String },
}

pub enum SelectError {
    Invalid,
    NotFound,
    Other { message: String },
}

pub enum AddTagError {
    NotFound,
    Conflict,
    Other { message: String },
}

pub enum RemoveTagError {
    ItemNotFound,
    TagNotFound,
    Conflict,
    Other { message: String },
}

pub enum SetPriorityError {
    NotFound,
    Other { message: String },
}
