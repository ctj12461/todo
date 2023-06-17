use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::entity::priority::Priority;
use crate::domain::entity::tag::{Tag, TagSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    id: u64,
    title: String,
    description: String,
    deadline: NaiveDateTime,
    tags: TagSet,
    priority: Priority,
}

impl Item {
    pub fn new(
        title: &str,
        description: &str,
        deadline: NaiveDateTime,
        tags: HashSet<Tag>,
        priority: Priority,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        title.hash(&mut hasher);
        description.hash(&mut hasher);
        deadline.hash(&mut hasher);
        priority.value().hash(&mut hasher);

        Self {
            id: hasher.finish(),
            title: title.to_owned(),
            description: description.to_owned(),
            deadline,
            tags,
            priority,
        }
    }

    #[inline]
    pub fn add_tag(&mut self, tag: &Tag) {
        self.tags.insert(tag.clone());
    }

    #[inline]
    pub fn remove_tag(&mut self, tag: &Tag) {
        self.tags.remove(tag);
    }

    #[inline]
    pub fn find_tag(&self, tag: &Tag) -> bool {
        self.tags.get(tag).is_some()
    }

    #[inline]
    pub fn get_all_tags(&self) -> Vec<Tag> {
        self.tags.iter().cloned().collect()
    }

    #[inline]
    pub fn is_expired(&self, time: NaiveDateTime) -> bool {
        time >= self.deadline
    }

    #[inline]
    pub fn upgrade(&mut self) {
        self.priority.upgrade();
    }

    #[inline]
    pub fn downgrade(&mut self) {
        self.priority.downgrade();
    }

    #[inline]
    pub fn priority(&self) -> &Priority {
        &self.priority
    }
}

impl PartialOrd for Item {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (&self.deadline, &self.priority, self.title.as_str()).partial_cmp(&(
            &other.deadline,
            &other.priority,
            other.title.as_str(),
        ))
    }
}
