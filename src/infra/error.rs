use crate::domain;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("domain erorr: {0}")]
    DomainError(domain::Error),

    #[error("system clock seems to have been rewound")]
    SystemClockRewound,
}
