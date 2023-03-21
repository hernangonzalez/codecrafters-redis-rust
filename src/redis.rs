mod cache;

use crate::command::Command;
use crate::response::{Builder, Response};
use std::sync::Mutex;
use std::time;

type Cache = cache::Cache<String, String>;

pub struct Redis {
    cache: Mutex<Cache>,
}

impl Redis {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(Cache::new()),
        }
    }

    pub fn handle(&self, cmd: &Command) -> Option<Response> {
        match cmd {
            Command::Ping => Some(Response::pong()),
            Command::Echo(message) => Some(Response::text(message)),
            Command::Get(key) => Some(self.handle_get(key)),
            Command::Set(key, value, delta) => Some(self.handle_set(key, value, delta)),
            Command::Unknown(cmd, args) => {
                println!("Skip unknown command: {cmd}, args: {args}");
                None
            }
        }
    }

    fn handle_get(&self, k: &String) -> Response {
        let mut cache = self.cache.lock().expect("unique access to cache");
        if let Some(value) = cache.get(k) {
            Response::text(value)
        } else {
            Response::error("Key not found")
        }
    }

    fn handle_set(&self, key: &String, value: &str, duration: &Option<time::Duration>) -> Response {
        let previous = {
            let mut cache = self.cache.lock().unwrap();
            cache.get(key).cloned()
        };
        let timeout = duration.map(|d| time::Instant::now() + d);
        let mut cache = self.cache.lock().expect("unique access to cache");

        cache.put(key.to_string(), value.to_string(), timeout);

        if let Some(value) = previous {
            Response::text(&value)
        } else {
            Response::ok()
        }
    }
}
