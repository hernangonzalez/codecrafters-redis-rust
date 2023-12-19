use crate::db::codec;
use anyhow::{Context, Result};
use bytes::BytesMut;
use std::io::{BufRead, BufReader};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

const REDIS_RDB: &[u8] = b"REDIS";
const REDIS_VER: &str = "0011";

#[allow(dead_code)]
#[derive(Debug)]
pub enum OpCode {
    EOF,
    SelectDB,
    ExpireTime,
    ExpireTimeMs,
    ResizeDB,
    Aux,
}

impl TryFrom<u8> for OpCode {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0xFF => Ok(Self::EOF),
            0xFE => Ok(Self::SelectDB),
            0xFD => Ok(Self::ExpireTime),
            0xFC => Ok(Self::ExpireTimeMs),
            0xFB => Ok(Self::ResizeDB),
            0xFA => Ok(Self::Aux),
            _ => Err(anyhow::anyhow!("Not an OpCode")),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AuxKey {
    RedisVer,
    RedisBits,
    CreationTime,
    UsedMem,
}

#[derive(Debug)]
pub struct Aux(String, String);

impl Aux {
    fn read(reader: &mut impl Read) -> Result<Aux> {
        let key = codec::string::read(reader)?;
        let value = codec::string::read(reader)?;
        Ok(Aux(key, value))
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Section {
    Head,
    Version(u8),
    Aux(Aux),
    Database(usize),
    Resize(usize, usize),
}

impl Section {
    fn from(code: OpCode, reader: &mut impl Read) -> Result<Self> {
        match code {
            OpCode::Aux => Aux::read(reader).map(Section::Aux),
            OpCode::SelectDB => codec::length::read(reader)
                .map(|len| len.into())
                .map(Section::Database),
            OpCode::ResizeDB => {
                let db_size = codec::length::read(reader)?;
                let exp_size = codec::length::read(reader)?;
                Ok(Section::Resize(db_size.into(), exp_size.into()))
            }
            other => Err(anyhow::anyhow!("To be implemented: {other:?}")),
        }
    }
}

#[derive(Debug)]
pub struct RedisFile(File, u32);

impl RedisFile {
    pub fn build_at(p: &Path) -> Result<RedisFile> {
        let dir = p.parent().context("dir")?;
        fs::create_dir_all(dir)?;
        match File::open(p) {
            Ok(file) => Self::sanity_check(file),
            Err(_) => Self::create_at(p),
        }
    }

    fn sanity_check(f: File) -> Result<RedisFile> {
        let mut file = f;
        let mut buffer = BytesMut::zeroed(REDIS_RDB.len());

        file.read_exact(&mut buffer)?;
        anyhow::ensure!(buffer.starts_with(REDIS_RDB));

        buffer.resize(4, 0);
        file.read_exact(&mut buffer)?;

        let ver = std::str::from_utf8(&buffer)?;
        Ok(RedisFile(file, ver.parse()?))
    }

    fn create_at(path: &Path) -> Result<RedisFile> {
        let mut file = File::create(path)?;
        file.write(REDIS_RDB)?;
        file.write(REDIS_VER.as_bytes())?;
        Ok(RedisFile(file, REDIS_VER.parse()?))
    }
}

impl IntoIterator for RedisFile {
    type Item = Section;
    type IntoIter = RedisFileReader;

    fn into_iter(self) -> Self::IntoIter {
        RedisFileReader(BufReader::new(self.0))
    }
}

pub struct RedisFileReader(BufReader<File>);

impl Iterator for RedisFileReader {
    type Item = Section;

    fn next(&mut self) -> Option<Self::Item> {
        let reader = &mut self.0;

        let buf = reader.fill_buf().ok()?;
        if buf.is_empty() {
            return None;
        }

        if let Ok(op_code) = OpCode::try_from(buf[0]) {
            reader.consume(1);
            Section::from(op_code, reader).ok()
        } else {
            None
        }
    }
}
