use anyhow::{Context, Result};
use std::time;

#[derive(PartialEq, Debug)]
pub enum ConfigKey {
    Dir,
    DbFilename,
}

impl TryFrom<&str> for ConfigKey {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "dir" => Ok(Self::Dir),
            "dbfilename" => Ok(Self::DbFilename),
            u => Err(anyhow::anyhow!("Unknown cmd: {u}")),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ConfigCmd {
    Get(ConfigKey),
}

impl TryFrom<&[&str]> for ConfigCmd {
    type Error = anyhow::Error;

    fn try_from(value: &[&str]) -> Result<Self> {
        let mut iter = value.iter();
        let cmd = iter.next().context("command")?;
        let param = iter.next().context("param")?;
        match *cmd {
            "GET" | "get" => Ok(Self::Get(ConfigKey::try_from(*param)?)),
            u => Err(anyhow::anyhow!("Unknown cmd: {u}")),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Get(String),
    Set(String, String, Option<time::Duration>),
    Config(ConfigCmd),
    Keys(String),
    Unknown(String, String),
}
