pub enum CommandName {
    Get,
    Stat,
    Unknown(String),
}

impl CommandName {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "get" => Self::Get,
            "stat" => Self::Stat,
            _ => Self::Unknown(s.to_string()),
        }
    }
}
