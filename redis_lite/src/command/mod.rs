pub enum Command {
    Get,
    Set,
    Ping,
    Invalid,
}

impl Command {
    pub fn get_command(value: &str) -> Command {
        println!("get command :{}", value);
        match value.as_bytes() {
            b"set" => Command::Set,
            b"get" => Command::Get,
            b"ping" => Command::Ping,
            _ => Command::Invalid,
        }
    }
}
