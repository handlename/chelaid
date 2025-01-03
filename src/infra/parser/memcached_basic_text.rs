pub enum Command {
    Get,
    Stat,
    Unknown,
}

pub struct CommandLine {
    command: Command,
}
