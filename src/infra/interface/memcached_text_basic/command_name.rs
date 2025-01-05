#[derive(Debug, Clone)]
pub enum CommandName {
    Get,
    Stat,
    Unknown(String),
}

impl CommandName {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => Self::Get,
            "STAT" => Self::Stat,
            _ => Self::Unknown(s.to_string()),
        }
    }
}

impl From<CommandName> for String {
    fn from(c: CommandName) -> Self {
        match c {
            CommandName::Get => "GET".to_string(),
            CommandName::Stat => "STAT".to_string(),
            CommandName::Unknown(s) => s,
        }
    }
}
