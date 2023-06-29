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
    summary: String,
    content: String,
    deadline: NaiveDateTime,
    tags: TagSet,
    priority: Priority,
}

impl Item {
    pub fn new(
        summary: &str,
        content: &str,
        deadline: NaiveDateTime,
        tags: HashSet<Tag>,
        priority: Priority,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        summary.hash(&mut hasher);
        content.hash(&mut hasher);
        deadline.hash(&mut hasher);

        Self {
            id: hasher.finish(),
            summary: summary.to_owned(),
            content: content.to_owned(),
            deadline,
            tags,
            priority,
        }
    }

    #[cfg(test)]
    pub fn new_test() -> Self {
        Item::new(
            "Test",
            "This is content.",
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
    pub fn summary(&self) -> &str {
        &self.summary
    }

    #[inline]
    pub fn content(&self) -> &str {
        &self.content
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
        (&self.deadline, &other.priority, self.summary.as_str()).partial_cmp(&(
            &other.deadline,
            &self.priority,
            other.summary.as_str(),
        ))
    }
}
