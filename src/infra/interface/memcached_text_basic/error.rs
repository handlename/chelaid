#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("command is empty")]
    EmptyCommand,

    #[error("command arguments are empty: {0}")]
    InvalidArguments(String),

    #[error("unknown command: {0}")]
    UnknownCommand(String),
}
