use super::super::error::Error;
use color_eyre::eyre::Result;

use super::super::constant::TIMESTAMP_OFFSET;

/// Epoch time in millseconds.
#[derive(Debug, PartialOrd, Ord)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn new(v: u64) -> Result<Self, Error> {
        if v < TIMESTAMP_OFFSET {
            return Err(Error::TimestampTooSmall(v));
        }

        Ok(Self(v))
    }

    pub fn new_from_system_time(st: std::time::SystemTime) -> Result<Self, Error> {
        let mills = st
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        // TODO: overflow check
        Self::new(mills as u64)
    }

    pub fn now() -> Result<Self, Error> {
        Self::new_from_system_time(std::time::SystemTime::now())
    }
}

impl std::convert::From<Timestamp> for u64 {
    fn from(ts: Timestamp) -> u64 {
        ts.0
    }
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Timestamp {}

impl Clone for Timestamp {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
