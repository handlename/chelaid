#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown error: {0}")]
    Unknown(String),

    #[error("sequence is too large: {0}")]
    SequenceTooLarge(u32),
}
