use color_eyre::eyre::{eyre, Result};

use crate::domain::constant::TIMESTAMP_OFFSET;

#[derive(Debug)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn new(v: u64) -> Result<Self> {
        if v < TIMESTAMP_OFFSET {
            return Err(eyre!("Timestamp is too small"));
        }

        Ok(Self(v))
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

impl Clone for Timestamp {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
