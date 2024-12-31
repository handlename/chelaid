use color_eyre::eyre::{eyre, Result};

use crate::domain::constant::TIMESTAMP_OFFSET;

/// Epoch time in millseconds.
#[derive(Debug, PartialOrd, Ord)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn new(v: u64) -> Result<Self> {
        if v < TIMESTAMP_OFFSET {
            return Err(eyre!("Timestamp is too small"));
        }

        Ok(Self(v))
    }

    pub fn new_from_system_time(st: std::time::SystemTime) -> Result<Self> {
        let mills = st
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        // TODO: overflow check
        Self::new(mills as u64)
    }

    pub fn now() -> Result<Self> {
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
