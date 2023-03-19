#[derive(PartialEq, Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Get(String),
    Set(String, String),
    Unknown(String, String),
}
