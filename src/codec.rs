#[allow(dead_code)]
#[repr(u8)]
pub enum Kind {
    String = 0,
    List = 1,
    Set = 2,
    SortedSet = 3,
    Hash = 4,
    ZipMap = 9,
    ZipList = 10,
    IntSet = 11,
    SortedSetZipList = 12,
    HashMapZipList = 13,
    QuickList = 14,
}

pub const CRLF: &str = "\r\n";

pub mod encode {
    use super::*;

    pub fn array(items: &[&str]) -> String {
        let msg = format!("*{}{CRLF}", items.len());
        items
            .iter()
            .fold(msg, |acc, m| acc + &format!("+{m}{CRLF}"))
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
