use std::ops::Deref;

#[derive(PartialEq, Debug)]
pub struct Response(String);

pub trait Builder {
    fn pong() -> Self;
    fn text(inner: &str) -> Self;
    fn error(msg: &str) -> Self;
    fn ok() -> Self;
    fn null() -> Self;
    fn array(items: &[&str]) -> Self;
}

const CRLF: &str = "\r\n";

impl Builder for Response {
    fn pong() -> Self {
        Self::text("PONG")
    }

    fn text(inner: &str) -> Self {
        Response(format!("+{inner}{CRLF}"))
    }

    fn error(msg: &str) -> Self {
        Response(format!("-Error {msg}{CRLF}"))
    }

    fn ok() -> Self {
        Self::text("OK")
    }

    fn null() -> Self {
        Response("$-1\r\n".to_string())
    }

    fn array(items: &[&str]) -> Self {
        let mut msg = format!("*{}{CRLF}", items.len());
        let msg = items
            .iter()
            .fold(msg, |acc, m| acc + &format!("+{m}{CRLF}"));
        Response(msg)
    }
}

impl Deref for Response {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
