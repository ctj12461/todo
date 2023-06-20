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

        Self {
            id: hasher.finish(),
            title: title.to_owned(),
            description: description.to_owned(),
            deadline,
            tags,
            priority,
        }
    }

    #[cfg(test)]
    pub fn new_test() -> Self {
        Item::new(
            "Test",
            "This is description.",
            NaiveDateTime::parse_from_str("2023-06-17 23:20:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            TagSet::new(),
            0.try_into().unwrap(),
        )
    }

    #[inline]
    pub fn id(&self) -> u64 {
        self.id
    }

    #[inline]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[inline]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[inline]
    pub fn deadline(&self) -> &NaiveDateTime {
        &self.deadline
    }

    #[inline]
    pub fn add_tag(&mut self, tag: Tag) -> bool {
        self.tags.insert(tag)
    }

    #[inline]
    pub fn remove_tag(&mut self, tag: &Tag) -> bool {
        self.tags.remove(tag)
    }

    #[inline]
    pub fn find_tag(&self, tag: &Tag) -> bool {
        self.tags.get(tag).is_some()
    }

    #[inline]
    pub fn tags(&self) -> &TagSet {
        &self.tags
    }

    #[inline]
    pub fn is_expired(&self, time: NaiveDateTime) -> bool {
        time >= self.deadline
    }

    #[inline]
    pub fn priority(&self) -> &Priority {
        &self.priority
    }

    #[inline]
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }
}

impl PartialOrd for Item {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (&self.deadline, &other.priority, self.title.as_str()).partial_cmp(&(
            &other.deadline,
            &self.priority,
            other.title.as_str(),
        ))
    }
}
