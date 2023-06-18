mod memory;

use crate::domain::entity::{Item, TagSet};

use chrono::NaiveDateTime;
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
