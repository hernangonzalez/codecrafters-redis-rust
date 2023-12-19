mod cache;

use crate::db::{self, Database};
use crate::{
    command::{Command, ConfigCmd, ConfigKey},
    config::Config,
    response::{Builder, Response},
};
use anyhow::Result;
use std::{sync::Mutex, time};

type Cache = cache::Cache<String, String>;

pub struct Redis {
    cache: Mutex<Cache>,
    config: Config,
}

impl Redis {
    pub fn new(config: Config) -> Result<Self> {
        let cache = Mutex::new(Cache::new());
        Ok(Self { cache, config })
    }

    pub fn handle(&self, cmd: &Command, received_at: time::Instant) -> Option<Response> {
        match cmd {
            Command::Ping => Some(Response::pong()),
            Command::Echo(message) => Some(Response::text(message)),
            Command::Get(key) => self.handle_get(key),
            Command::Set(key, value, delta) => {
                let timeout = delta.map(|d| received_at + d);
                Some(self.handle_set(key, value, timeout))
            }
            Command::Config(cmd) => Some(self.handle_config(cmd)),
            Command::Keys(op) => self.handle_keys(op).ok(),
            Command::Unknown(cmd, args) => {
                println!("Skip unknown command: {cmd}, args: {args}");
                None
            }
        }
    }

    fn handle_keys(&self, _op: &str) -> Result<Response> {
        let db = db::open_at(&self.config.local_store_path())?;
        let keys = db.all_keys();
        let keys: Vec<_> = keys.iter().map(String::as_str).collect();
        Ok(Response::array(&keys))
    }

    fn handle_config(&self, cmd: &ConfigCmd) -> Response {
        match cmd {
            ConfigCmd::Get(key) => match key {
                ConfigKey::Dir => Response::array(&["dir", &self.config.dir.to_str().unwrap()]),
                ConfigKey::DbFilename => Response::array(&["dbfilename", &self.config.db_filename]),
            },
        }
    }

    fn handle_get(&self, k: &String) -> Option<Response> {
        let mut cache = self.cache.lock().expect("unique access to cache");
        if let Ok(cached) = cache.value(k) {
            return Some(Response::text(cached));
        }

        let db = db::open_at(&self.config.local_store_path()).ok()?;
        if let Some(val) = db.find(k) {
            return Some(Response::text(&val));
        }

        None
    }

    fn handle_set(&self, key: &String, value: &str, timeout: Option<time::Instant>) -> Response {
        let previous = {
            let mut cache = self.cache.lock().unwrap();
            cache.value(key).cloned()
        };
        let mut cache = self.cache.lock().expect("unique access to cache");

        cache.put(key.to_string(), value.to_string(), timeout);

        if let Ok(value) = previous {
            Response::text(&value)
        } else {
            Response::ok()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        thread,
        time::{Duration, Instant},
    };

    #[test]
    fn test_set_get() {
        let dur = Duration::from_millis(100);
        let set = Command::Set("k".into(), "v".into(), Some(dur));
        let get = Command::Get("k".into());
        let cfg = Config::default();
        let sut = Redis::new(cfg).unwrap();
        let now = Instant::now();
        assert_eq!(sut.handle(&set, now), Some(Response::ok()));
        assert_eq!(sut.handle(&get, now), Some(Response::text("v")));
        thread::sleep(dur);
        assert_eq!(sut.handle(&get, Instant::now()), Some(Response::null()));
    }
}
