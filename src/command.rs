use std::str::Lines;

pub enum Command {
    Ping,
    Echo(String),
}

impl TryFrom<&mut Lines<'_>> for Command {
    type Error = String;

    fn try_from(lines: &mut Lines<'_>) -> std::result::Result<Self, Self::Error> {
        let head = lines.next().ok_or("missing head")?;
        assert!(head.starts_with('*'), "Invalid start for command");

        let len = lines.next().ok_or("missing command length")?;
        assert!(len.starts_with('$'), "Invalid start for length");

        let command = lines.next().ok_or("missing command")?;
        let command = command.to_uppercase();
        let command = match command.as_str() {
            "PING" => Command::Ping,
            "ECHO" => {
                let len = lines.next().ok_or("missing message length")?;
                assert!(len.starts_with('$'), "Invalid start for length");

                let text = lines.next().ok_or("missing echo text")?;
                Command::Echo(text.into())
            }
            u => return Err(format!("Unknown command: {u}")),
        };

        Ok(command)
    }
}
