mod codec;
mod file;

use anyhow::Result;
use codec::Value;
use file::{RedisFile, Section};
use std::path::Path;

pub trait Database {
    fn all_keys(self) -> Vec<String>;
    fn find(self, key: &str) -> Option<String>;
}

pub fn open_at(path: &Path) -> Result<impl Database> {
    let file = RedisFile::build_at(path)?;
    Ok(file)
}

impl RedisFile {
    fn all_entries(self) -> impl Iterator<Item = (String, Value)> {
        self.into_iter().flat_map(|s| match s {
            Section::Entry(_, k, v) => Some((k, v)),
            _ => None,
        })
    }
}

impl Database for RedisFile {
    fn all_keys(self) -> Vec<String> {
        self.all_entries().map(|(k, _)| k).collect()
    }

    fn find(self, key: &str) -> Option<String> {
        self.all_entries()
            .find(|(k, _)| k == key)
            .map(|(_, v)| match v {
                Value::String(s) => s.clone(),
            })
    }
}
