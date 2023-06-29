use std::collections::hash_map::{Entry, HashMap};

use crate::domain::entity::{Item, Priority, TagSet};

use super::{
    AddError, AddTagError, GetError, Pool, RemoveError, RemoveTagError, SelectError,
    SetPriorityError,
};

pub struct MemoryPool {
    items: HashMap<u64, Item>,
}

impl MemoryPool {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn clone_inner(&self) -> HashMap<u64, Item> {
        self.items.clone()
    }
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}

impl From<HashMap<u64, Item>> for MemoryPool {
    fn from(value: HashMap<u64, Item>) -> Self {
        Self { items: value }
    }
}

impl Pool for MemoryPool {
    fn add(&mut self, item: Item) -> Result<u64, AddError> {
        let id = item.id();

        if let Entry::Vacant(e) = self.items.entry(id) {
            e.insert(item);
            Ok(id)
        } else {
            Err(AddError::Conflict)
        }
    }

    fn remove(&mut self, id: u64) -> Result<Item, RemoveError> {
        if let Some(item) = self.items.remove(&id) {
            Ok(item)
        } else {
            Err(RemoveError::NotFound)
        }
    }

    fn get(&self, id: u64) -> Result<Item, GetError> {
        if let Some(item) = self.items.get(&id) {
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

        let mut res = self
            .items
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

    fn add_tag(&mut self, id: u64, tags: TagSet) -> Result<(), AddTagError> {
        if let Some(item) = self.items.get_mut(&id) {
            let mut not_existed = true;

            for tag in tags.into_iter() {
                if !item.add_tag(tag) {
                    not_existed = false;
                }
            }

            if not_existed {
                Ok(())
            } else {
                Err(AddTagError::Conflict)
            }
        } else {
            Err(AddTagError::NotFound)
        }
    }

    fn remove_tag(&mut self, id: u64, tags: TagSet) -> Result<(), RemoveTagError> {
        if let Some(item) = self.items.get_mut(&id) {
            let mut all_existed = true;

            for tag in tags {
                if !item.remove_tag(&tag) {
                    all_existed = false;
                }
            }

            if all_existed {
                Ok(())
            } else {
                Err(RemoveTagError::TagNotFound)
            }
        } else {
            Err(RemoveTagError::ItemNotFound)
        }
    }

    fn set_priority(&mut self, id: u64, priority: Priority) -> Result<(), SetPriorityError> {
        if let Some(item) = self.items.get_mut(&id) {
            item.set_priority(priority);
            Ok(())
        } else {
            Err(SetPriorityError::NotFound)
        }
    }

    fn clear(&mut self) {
        self.items.clear();
    }
}
