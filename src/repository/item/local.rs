use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, BufWriter, Error as IoError, Read, Write};
use std::path::PathBuf;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use snafu::prelude::*;

use crate::domain::entity::{Item, Priority, TagSet};
use crate::repository::item::memory::MemoryPool;

use super::{
    AddError, AddTagError, GetError, Pool, RemoveError, RemoveTagError, SelectError,
    SetPriorityError,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct RawItem {
    pub title: String,
    pub description: String,
    pub deadline: NaiveDateTime,
    pub tags: TagSet,
    pub priority: Priority,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Data {
    items: HashSet<RawItem>,
}

pub struct LocalPool {
    repo: MemoryPool,
    path: PathBuf,
}

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum InitError {
    #[snafu(display("Failed to load storage due to invalid JSON content"))]
    Invalid { source: SerdeError },
    #[snafu(display("Failed to open storage: {source}"))]
    Open { source: IoError },
    #[snafu(display("Failed to read items: {source}"))]
    Read { source: IoError },
}

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum SyncError {
    #[snafu(display("Failed to dump items to JSON: {source}"))]
    Dump { source: SerdeError },
    #[snafu(display("Failed to open storage: {source}"))]
    Open { source: IoError },
    #[snafu(display("Failed to write items: {source}"))]
    Write { source: IoError },
}

impl Hash for RawItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.title.hash(state);
        self.description.hash(state);
        self.deadline.hash(state);
    }
}

impl From<RawItem> for Item {
    fn from(value: RawItem) -> Self {
        Item::new(
            value.title.as_str(),
            value.description.as_str(),
            value.deadline,
            value.tags,
            value.priority,
        )
    }
}

impl From<Item> for RawItem {
    fn from(value: Item) -> Self {
        Self {
            title: value.title().to_owned(),
            description: value.description().to_owned(),
            deadline: *value.deadline(),
            tags: value.tags().clone(),
            priority: value.priority().clone(),
        }
    }
}

impl From<Data> for HashMap<u64, Item> {
    fn from(value: Data) -> Self {
        value
            .items
            .into_iter()
            .map(|item| {
                let item: Item = Into::into(item);
                (item.id(), item)
            })
            .collect()
    }
}

impl From<HashMap<u64, Item>> for Data {
    fn from(value: HashMap<u64, Item>) -> Self {
        let items = value.into_values().map(|item| item.into()).collect();
        Data { items }
    }
}

impl LocalPool {
    pub fn open(path: PathBuf) -> Result<Self, InitError> {
        let json = Self::read_file(path.clone())?;
        let data = Self::deserialize(json)?;

        Ok(Self {
            repo: MemoryPool::from(HashMap::from(data)),
            path,
        })
    }

    fn read_file(path: PathBuf) -> Result<String, InitError> {
        let file = match OpenOptions::new().read(true).create(true).open(path) {
            Ok(file) => file,
            Err(err) => return Err(InitError::Open { source: err }),
        };

        let mut reader = BufReader::new(file);
        let mut json = String::new();

        reader
            .read_to_string(&mut json)
            .map(|_| json)
            .map_err(|err| InitError::Read { source: err })
    }

    fn sync_file(path: PathBuf, json: String) -> Result<(), SyncError> {
        let file = match OpenOptions::new().write(true).create(true).open(path) {
            Ok(file) => file,
            Err(err) => return Err(SyncError::Open { source: err }),
        };

        let mut writer = BufWriter::new(file);
        writer
            .write_all(json.as_bytes())
            .map_err(|err| SyncError::Write { source: err })
    }

    pub fn sync(&self) -> Result<(), SyncError> {
        let data: Data = self.repo.clone_inner().into();
        let json = Self::serialize(data)?;
        Self::sync_file(self.path.clone(), json)
    }

    fn deserialize(json: String) -> Result<Data, InitError> {
        if !json.is_empty() {
            serde_json::from_str::<Data>(json.as_str())
                .map_err(|err| InitError::Invalid { source: err })
        } else {
            Ok(Data {
                items: HashSet::new(),
            })
        }
    }

    fn serialize(data: Data) -> Result<String, SyncError> {
        serde_json::to_string(&data).map_err(|err| SyncError::Dump { source: err })
    }
}

impl Drop for LocalPool {
    fn drop(&mut self) {
        if let Err(err) = self.sync() {
            panic!("{err}");
        }
    }
}

impl Pool for LocalPool {
    fn add(&mut self, item: Item) -> Result<u64, AddError> {
        self.repo.add(item)
    }

    fn remove(&mut self, id: u64) -> Result<Item, RemoveError> {
        self.repo.remove(id)
    }

    fn get(&self, id: u64) -> Result<Item, GetError> {
        self.repo.get(id)
    }

    fn select(
        &self,
        tags: TagSet,
        before: Option<NaiveDateTime>,
        after: Option<NaiveDateTime>,
    ) -> Result<Vec<Item>, SelectError> {
        self.repo.select(tags, before, after)
    }

    fn add_tag(&mut self, id: u64, tags: TagSet) -> Result<(), AddTagError> {
        self.repo.add_tag(id, tags)
    }

    fn remove_tag(&mut self, id: u64, tags: TagSet) -> Result<(), RemoveTagError> {
        self.repo.remove_tag(id, tags)
    }

    fn set_priority(&mut self, id: u64, priority: Priority) -> Result<(), SetPriorityError> {
        self.repo.set_priority(id, priority)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_correct_data_when_deserializing_from_a_string() {
        let json = serde_json::json!({
            "items": [
                {
                    "title": "1",
                    "description": "Test 1",
                    "deadline": "2023-06-17T23:20:00",
                    "tags": [],
                    "priority": 1
                },
                {
                    "title": "2",
                    "description": "Test 2",
                    "deadline": "2023-06-17T23:20:00",
                    "tags": [],
                    "priority": 2
                },
                {
                    "title": "3",
                    "description": "Test 3",
                    "deadline": "2023-06-17T23:20:00",
                    "tags": [],
                    "priority": 3
                }
            ]
        })
        .to_string();

        if let Ok(data) = LocalPool::deserialize(json) {
            let items = [
                RawItem {
                    title: "1".to_owned(),
                    description: "Test 1".to_owned(),
                    deadline: get_deadline(),
                    tags: TagSet::new(),
                    priority: 1.try_into().unwrap(),
                },
                RawItem {
                    title: "2".to_owned(),
                    description: "Test 2".to_owned(),
                    deadline: get_deadline(),
                    tags: TagSet::new(),
                    priority: 2.try_into().unwrap(),
                },
                RawItem {
                    title: "3".to_owned(),
                    description: "Test 3".to_owned(),
                    deadline: get_deadline(),
                    tags: TagSet::new(),
                    priority: 3.try_into().unwrap(),
                },
            ]
            .into_iter()
            .collect();
            assert_eq!(data, Data { items });
        } else {
            unreachable!();
        }
    }

    #[test]
    fn it_should_return_a_empty_hash_set_when_deserializing_from_a_empty_string() {
        if let Ok(data) = LocalPool::deserialize(String::new()) {
            assert_eq!(
                data,
                Data {
                    items: HashSet::new(),
                }
            );
        } else {
            unreachable!();
        }
    }

    #[test]
    fn it_should_return_invalid_error_when_deserializing_from_an_irrelevant_string() {
        let json = serde_json::json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        })
        .to_string();

        assert!(matches!(
            LocalPool::deserialize(json),
            Err(InitError::Invalid { source: _ })
        ));
    }

    #[inline]
    fn get_deadline() -> NaiveDateTime {
        NaiveDateTime::parse_from_str("2023-06-17 23:20:00", "%Y-%m-%d %H:%M:%S").unwrap()
    }
}
