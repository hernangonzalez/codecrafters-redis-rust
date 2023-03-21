use std::time;

#[derive(PartialEq, Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Get(String),
    Set(String, String, Option<time::Duration>),
    Unknown(String, String),
}
