use super::Command;
use anyhow::{Context, Result};
use std::str::Lines;

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
    let args: Vec<_> = lines
        .filter(|s| !s.starts_with('$'))
        .map(|s| s.to_string())
        .collect();

    let command = match command.as_str() {
        "PING" => Command::Ping,
        "ECHO" => {
            let text = args.join(" ");
            Command::Echo(text)
        }
        _ => Command::Unknown(command),
    };

    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ECHO: &str = "*2\r\n$4\r\necho\r\n$12\r\ntoma mensaje\r\n";

    #[test]
    fn test_scan() {
        let buffer = ECHO;
        let cmd = scan(buffer);
        assert_eq!(*cmd, [Command::Echo("toma mensaje".to_string())]);
    }
}
