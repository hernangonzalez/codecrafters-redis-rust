use super::Command;
use anyhow::{Context, Result};
use std::str::Lines;
use std::time;

// For Simple Strings, the first byte of the reply is "+"
// For Errors, the first byte of the reply is "-"
// For Integers, the first byte of the reply is ":"
// For Bulk Strings, the first byte of the reply is "$"
// For Arrays, the first byte of the reply is "*"

pub fn scan(buffer: &str) -> Box<[Command]> {
    let mut cmds = Vec::new();
    let chunks = buffer.split('*').filter(|i| !i.is_empty());
    for chunk in chunks {
        let lines = chunk.lines();
        match scan_command(lines) {
            Ok(c) => cmds.push(c),
            Err(e) => println!("{e}"),
        }
    }
    cmds.into_boxed_slice()
}

fn scan_command(lines: Lines) -> Result<Command> {
    let mut lines = lines;

    let arg_count = lines.next().context("arg count")?.parse::<usize>()?;
    assert_ne!(arg_count, 0);

    let line_head = lines.next().context("command head")?;
    assert!(line_head.starts_with('$'));

    let command = lines.next().context("command")?.to_uppercase();
    let mut args = lines.filter(|s| !s.starts_with('$'));

    let command = match command.as_str() {
        "PING" => Command::Ping,
        "GET" => {
            let key = args.next().context("get key")?;
            Command::Get(key.to_string())
        }
        "SET" => {
            let key = args.next().context("set key")?.to_string();
            let value = args.next().context("set value")?.to_string();
            let mut timeout = None;
            if args.next().map(|s| s.to_uppercase()) == Some("PX".to_string()) {
                let param = args.next().context("PX miliseconds")?.parse::<u64>()?;
                timeout = Some(time::Duration::from_millis(param));
            }
            Command::Set(key, value, timeout)
        }
        "ECHO" => {
            let all: Vec<_> = args.collect();
            Command::Echo(all.join(""))
        }
        _ => {
            let all: Vec<_> = args.collect();
            Command::Unknown(command, all.join(" "))
        }
    };

    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan() {
        const ECHO: &str = "*2\r\n$4\r\necho\r\n$12\r\ntoma mensaje\r\n";
        let cmd = scan(ECHO);
        assert_eq!(*cmd, [Command::Echo("toma mensaje".to_string())]);
    }

    #[test]
    fn test_scan_set() {
        const SET: &str = "*3\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$5\r\nHello\r\n";
        let cmd = scan(SET);
        assert_eq!(
            *cmd,
            [Command::Set("mykey".to_string(), "Hello".to_string(), None)]
        );
    }

    #[test]
    fn test_scan_set_timeout() {
        const SET: &str =
            "*2\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$5\r\nHello\r\n$2\r\nPX\r\n$3\r\n100\r\n";
        let cmd = scan(SET);
        assert_eq!(
            *cmd,
            [Command::Set(
                "mykey".to_string(),
                "Hello".to_string(),
                Some(time::Duration::from_millis(100))
            )]
        );
    }
}
