use std::collections::hash_map::{Entry, HashMap};
use std::sync::Mutex;

use crate::domain::entity::{Item, Priority, TagSet};

use super::{
    AddError, AddTagError, GetError, RemoveError, RemoveTagError, Repository, SelectError,
    SetPriorityError,
};

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

    fn get(&self, id: u64) -> Result<Item, GetError> {
        let items = match self.items.lock() {
            Ok(items) => items,
            Err(err) => {
                return Err(GetError::Other {
                    message: err.to_string(),
                })
            }
        };

        if let Some(item) = items.get(&id) {
            Ok(item.clone())
        } else {
            Err(GetError::NotFound)
        }
    }

    fn select(
        &self,
        tags: TagSet,
        before: Option<chrono::NaiveDateTime>,
        after: Option<chrono::NaiveDateTime>,
    ) -> Result<Vec<Item>, SelectError> {
        if before.is_some() && after.is_some() && before.unwrap() > after.unwrap() {
            return Err(SelectError::Invalid);
        }

        let items = match self.items.lock() {
            Ok(items) => items,
            Err(err) => {
                return Err(SelectError::Other {
                    message: err.to_string(),
                })
            }
        };

        let mut res = items
            .values()
            .filter(|item| tags.is_subset(item.tags()))
            .filter(|item| before.is_none() || *item.deadline() <= before.unwrap())
            .filter(|item| after.is_none() || *item.deadline() >= after.unwrap())
            .cloned()
            .collect::<Vec<_>>();

        res.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        if !res.is_empty() {
            Ok(res)
        } else {
            Err(SelectError::NotFound)
        }
    }

    fn add_tag(&self, id: u64, tags: TagSet) -> Result<(), AddTagError> {
        let mut items = match self.items.lock() {
            Ok(items) => items,
            Err(err) => {
                return Err(AddTagError::Other {
                    message: err.to_string(),
                })
            }
        };

        if let Some(item) = items.get_mut(&id) {
            let not_existed = tags.into_iter().map(|tag| item.add_tag(tag)).all(|v| v);

            if not_existed {
                Ok(())
            } else {
                Err(AddTagError::Conflict)
            }
        } else {
            Err(AddTagError::NotFound)
        }
    }

    fn remove_tag(&self, id: u64, tags: TagSet) -> Result<(), RemoveTagError> {
        let mut items = match self.items.lock() {
            Ok(items) => items,
            Err(err) => {
                return Err(RemoveTagError::Other {
                    message: err.to_string(),
                })
            }
        };

        if let Some(item) = items.get_mut(&id) {
            let all_existed = tags.into_iter().map(|tag| item.remove_tag(&tag)).all(|v| v);

            if all_existed {
                Ok(())
            } else {
                Err(RemoveTagError::TagNotFound)
            }
        } else {
            Err(RemoveTagError::ItemNotFound)
        }
    }

    fn set_priority(&self, id: u64, priority: Priority) -> Result<(), SetPriorityError> {
        let mut items = match self.items.lock() {
            Ok(items) => items,
            Err(err) => {
                return Err(SetPriorityError::Other {
                    message: err.to_string(),
                })
            }
        };

        if let Some(item) = items.get_mut(&id) {
            item.set_priority(priority);
            Ok(())
        } else {
            Err(SetPriorityError::NotFound)
        }
    }
}
