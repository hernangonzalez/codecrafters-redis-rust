use anyhow::Result;
use std::fs::File;
use std::path::Path;

pub struct LocalStore(File);

pub trait Database {
    fn all_keys(&self) -> Vec<String>;
}

impl LocalStore {
    pub fn open_at(path: &Path) -> Result<Self> {
        let file = file::build_at(path)?;
        Ok(LocalStore(file))
    }
}

impl Database for LocalStore {
    fn all_keys(&self) -> Vec<String> {
        todo!()
    }
}

mod file {
    use super::*;
    use anyhow::Context;
    use bytes::BytesMut;
    use std::{
        fs::{self, File},
        io::{Read, Write},
    };

    const REDIS_RDB: &[u8] = b"REDIS0003";

    #[allow(dead_code)]
    #[repr(u8)]
    enum OpCode {
        EOF = 0xFF,
        SelectDB = 0xFE,
        ExpireTime = 0xFD,
        ExpireTimeMs = 0xFC,
        ResizeDB = 0xFB,
        Aux = 0xFA,
    }

    pub fn build_at(p: &Path) -> Result<File> {
        let dir = p.parent().context("dir")?;
        fs::create_dir_all(dir)?;
        match File::open(p) {
            Ok(file) => file::sanity_check(file),
            Err(_) => file::create_at(p),
        }
    }

    pub fn sanity_check(f: File) -> Result<File> {
        let mut file = f;
        let mut buffer = BytesMut::zeroed(REDIS_RDB.len());
        file.read(&mut buffer)?;
        anyhow::ensure!(buffer.starts_with(REDIS_RDB));
        Ok(file)
    }

    pub fn create_at(path: &Path) -> Result<File> {
        let mut file = File::create(path)?;
        file.write(REDIS_RDB)?;
        Ok(file)
    }
}
