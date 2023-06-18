mod memory;

use snafu::prelude::*;

use crate::domain::entity::Item;

pub use memory::MemoryRepositry;

pub trait Repository: Send + Sync {
    fn add(&self, item: Item) -> Result<u64, AddError>;
}

#[derive(Debug, Snafu)]
pub enum AddError {
    Conflict,
    #[snafu(display("{message}"))]
    Other {
        message: String,
    },
}
