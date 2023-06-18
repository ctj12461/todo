mod memory;

use crate::domain::entity::Item;

pub use memory::MemoryRepositry;

pub trait Repository: Send + Sync {
    fn add(&self, item: Item) -> Result<u64, AddError>;

    fn remove(&self, id: u64) -> Result<Item, RemoveError>;
}

pub enum AddError {
    Conflict,
    Other { message: String },
}

pub enum RemoveError {
    NotFound,
    Other { message: String },
}
