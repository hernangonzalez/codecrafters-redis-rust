use crate::proto::encode;

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

impl From<String> for Response {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl<'a> Into<&'a [u8]> for &'a Response {
    fn into(self) -> &'a [u8] {
        self.0.as_bytes()
    }
}

impl Builder for Response {
    fn pong() -> Self {
        encode::text("PONG").into()
    }

    fn text(inner: &str) -> Self {
        encode::text(inner).into()
    }

    fn error(msg: &str) -> Self {
        encode::error(msg).into()
    }

    fn ok() -> Self {
        encode::text("OK").into()
    }

    fn null() -> Self {
        encode::null().into()
    }

    fn array(items: &[&str]) -> Self {
        encode::array(items).into()
    }
}
