#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("command is empty")]
    EmptyCommand,

    #[error("command arguments are empty")]
    EmptyArguments,
}
