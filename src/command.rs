use std::collections::VecDeque;

pub enum Command {
    Ping,
    Echo(String),
}

pub fn extract(lines: &mut VecDeque<String>) -> std::result::Result<Command, String> {
    let mut head = lines.pop_front().ok_or("missing head")?;
    assert!(head.starts_with('*'), "Invalid start for command");
    head.remove(0);

    let arg_count = head.parse::<usize>().map_err(|e| e.to_string())?;
    let mut lines: VecDeque<_> = lines.drain(..arg_count * 2).collect();

    let len = lines.pop_front().ok_or("missing command length")?;
    assert!(len.starts_with('$'), "Invalid start for length");

    let command = lines.pop_front().ok_or("missing command")?.to_uppercase();
    let command = match command.as_str() {
        "PING" => Command::Ping,
        "ECHO" => {
            let len = lines.pop_front().ok_or("missing message length")?;
            assert!(len.starts_with('$'), "Invalid start for length");

            let text = lines.pop_front().ok_or("missing echo text")?;
            Command::Echo(text)
        }
        u => return Err(format!("Unknown command: {u}")),
    };

    Ok(command)
}
