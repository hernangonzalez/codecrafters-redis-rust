mod codec;
mod file;

use anyhow::Result;
use file::{RedisFile, Section};
use std::path::Path;

pub trait Database {
    fn all_keys(self) -> Vec<String>;
}

pub fn open_at(path: &Path) -> Result<impl Database> {
    let file = RedisFile::build_at(path)?;
    Ok(file)
}

impl Database for RedisFile {
    fn all_keys(self) -> Vec<String> {
        self.into_iter()
            .flat_map(|s| match s {
                Section::Entry(k) => Some(k),
                _ => None,
            })
            .collect()
    }
}
