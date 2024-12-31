#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown error: {0}")]
    Unknown(std::sync::Arc<String>),

    #[error("sequence is too large: {0}")]
    SequenceTooLarge(u32),

    #[error("timestamp is too small: {0}")]
    TimestampTooSmall(u64),

    #[error("worker id is too large: {0}")]
    WorkerIDTooLarge(u32),
}
