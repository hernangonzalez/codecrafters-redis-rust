use std::ops::Deref;

pub struct Response(String);

pub trait Builder {
    fn pong() -> Self;
    fn text(inner: &str) -> Self;
    fn error(msg: &str) -> Self;
    fn ok() -> Self;
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
}

impl Deref for Response {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
