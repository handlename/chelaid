#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("failed to craete thread: {0}")]
    ThreadCreateFailed(String),
}
