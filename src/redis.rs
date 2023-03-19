mod cache;

use crate::command::Command;
use crate::response::{Builder, Response};
use cache::Cache;

pub struct Redis {
    cache: Cache<String, String>,
}

impl Redis {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(),
        }
    }

    pub fn handle(&self, cmd: &Command) -> Option<Response> {
        match cmd {
            Command::Ping => Some(Response::pong()),
            Command::Echo(message) => Some(Response::text(message)),
            Command::Get(key) => Some(self.handle_get(key)),
            Command::Set(key, value) => Some(self.handle_set(key, value)),
            Command::Unknown(cmd, args) => {
                println!("Skip unknown command: {cmd}, args: {args}");
                None
            }
        }
    }

    fn handle_get(&self, _k: &str) -> Response {
        Response::error("Not found")
    }

    fn handle_set(&self, _k: &str, _v: &str) -> Response {
        Response::text("Ok")
    }
}
