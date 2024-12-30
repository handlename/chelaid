use color_eyre::eyre::{eyre, Result};

use crate::domain::constant::SEQUENCE_BITS;

pub struct Sequence(u32);

impl Sequence {
    pub fn new(v: u32) -> Result<Sequence> {
        if (1 << SEQUENCE_BITS) <= v {
            return Err(eyre!("Sequence is too large"));
        }

        Ok(Sequence(v))
    }
}

impl std::convert::From<Sequence> for u32 {
    fn from(seq: Sequence) -> u32 {
        seq.0
    }
}

impl std::convert::From<Sequence> for u64 {
    fn from(seq: Sequence) -> u64 {
        seq.0 as u64
    }
}
