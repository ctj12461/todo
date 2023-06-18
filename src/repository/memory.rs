use std::collections::hash_map::{Entry, HashMap};
use std::sync::Mutex;

use crate::domain::entity::Item;

use super::{AddError, RemoveError, Repository};

pub struct MemoryRepositry {
    items: Mutex<HashMap<u64, Item>>,
}

impl MemoryRepositry {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for MemoryRepositry {
    fn default() -> Self {
        Self::new()
    }
}

impl From<HashMap<u64, Item>> for MemoryRepositry {
    fn from(value: HashMap<u64, Item>) -> Self {
        Self {
            items: Mutex::new(value),
        }
    }
}

impl Repository for MemoryRepositry {
    fn add(&self, item: Item) -> Result<u64, AddError> {
        let mut items = match self.items.lock() {
            Ok(items) => items,
            Err(err) => {
                return Err(AddError::Other {
                    message: err.to_string(),
                })
            }
        };

        let id = item.id();

        if let Entry::Vacant(e) = items.entry(id) {
            e.insert(item);
            Ok(id)
        } else {
            Err(AddError::Conflict)
        }
    }

    fn remove(&self, id: u64) -> Result<Item, RemoveError> {
        let mut items = match self.items.lock() {
            Ok(items) => items,
            Err(err) => {
                return Err(RemoveError::Other {
                    message: err.to_string(),
                })
            }
        };

        if let Some(item) = items.remove(&id) {
            Ok(item)
        } else {
            Err(RemoveError::NotFound)
        }
    }
}
