#[derive(Debug, Clone, PartialEq)]
pub enum CommandName {
    Get,
    Stat,
    End,
    Unknown(String),
}

impl From<&str> for CommandName {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => Self::Get,
            "STAT" => Self::Stat,
            "END" => Self::End,
            _ => Self::Unknown(s.to_string()),
        }
    }
}

impl From<CommandName> for String {
    fn from(c: CommandName) -> Self {
        match c {
            CommandName::Get => "GET".to_string(),
            CommandName::Stat => "STAT".to_string(),
            CommandName::End => "END".to_string(),
            CommandName::Unknown(s) => s,
        }
    }
}
