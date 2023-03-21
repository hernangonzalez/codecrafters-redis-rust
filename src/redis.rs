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

    pub fn handle(&self, cmd: &Command, received_at: time::Instant) -> Option<Response> {
        match cmd {
            Command::Ping => Some(Response::pong()),
            Command::Echo(message) => Some(Response::text(message)),
            Command::Get(key) => Some(self.handle_get(key)),
            Command::Set(key, value, delta) => {
                let timeout = delta.map(|d| received_at + d);
                Some(self.handle_set(key, value, timeout))
            }
            Command::Unknown(cmd, args) => {
                println!("Skip unknown command: {cmd}, args: {args}");
                None
            }
        }
    }

    fn handle_get(&self, k: &String) -> Response {
        let mut cache = self.cache.lock().expect("unique access to cache");
        match cache.value(k) {
            Ok(value) => Response::text(value),
            e => {
                println!("Cache value error: {e:?}");
                Response::null()
            }
        }
    }

    fn handle_set(&self, key: &String, value: &str, timeout: Option<time::Instant>) -> Response {
        dbg!(key, value, timeout);
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
        let sut = Redis::new();
        let now = Instant::now();
        assert_eq!(sut.handle(&set, now), Some(Response::ok()));
        assert_eq!(sut.handle(&get, now), Some(Response::text("v")));
        thread::sleep(dur);
        assert_eq!(sut.handle(&get, Instant::now()), Some(Response::null()));
    }
}
