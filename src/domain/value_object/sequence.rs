use color_eyre::eyre::{eyre, Result};

use crate::domain::constant::SEQUENCE_BITS;

#[derive(Debug)]
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

impl PartialEq for Sequence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Clone for Sequence {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tests = vec![
            (0, true),
            (1 << SEQUENCE_BITS, false),
            ((1 << SEQUENCE_BITS) - 1, true),
        ];

        for (v, valid) in tests {
            match Sequence::new(v) {
                Ok(seq) => {
                    assert!(valid);
                    assert_eq!(u32::from(seq), v);
                }
                Err(_) => assert!(!valid),
            }
        }
    }
}
