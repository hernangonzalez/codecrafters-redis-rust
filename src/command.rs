#[derive(PartialEq, Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Unknown(String, String),
}
