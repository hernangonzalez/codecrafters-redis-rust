// For Simple Strings, the first byte of the reply is "+"
// For Errors, the first byte of the reply is "-"
// For Integers, the first byte of the reply is ":"
// For Bulk Strings, the first byte of the reply is "$"
// For Arrays, the first byte of the reply is "*"

pub const CRLF: &str = "\r\n";

pub mod encode {
    use super::*;

    pub fn array(items: &[&str]) -> String {
        let msg = format!("*{}{CRLF}", items.len());
        items
            .iter()
            .fold(msg, |acc, m| acc + &format!("${}{CRLF}{m}{CRLF}", m.len()))
    }

    pub fn text(s: &str) -> String {
        format!("+{s}{CRLF}")
    }

    pub fn error(e: &str) -> String {
        format!("-Error {e}{CRLF}")
    }

    pub fn null() -> String {
        format!("$-1{CRLF}")
    }
}

pub mod decode {
    pub fn array(s: &str) -> impl Iterator<Item = &str> {
        s.lines().skip(1).filter(|s| !s.starts_with('$'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_array() {
        let str = encode::array(&["ECHO", "hey"]);
        assert_eq!(str, "*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n");
    }

    #[test]
    fn test_decode_array() {
        let vec = decode::array("*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n").collect::<Vec<_>>();
        assert_eq!(&vec, &["ECHO", "hey"]);
    }
}
