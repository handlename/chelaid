use color_eyre::eyre::{eyre, Result};

use crate::domain::constant::WORKER_ID_BITS;

#[derive(Debug)]
pub struct WorkerID(u32);

impl WorkerID {
    pub fn new(v: u32) -> Result<WorkerID> {
        if (1 << WORKER_ID_BITS) <= v {
            return Err(eyre!("WorkerID is too large"));
        }

        Ok(WorkerID(v))
    }
}

impl std::convert::From<WorkerID> for u32 {
    fn from(worker_id: WorkerID) -> u32 {
        worker_id.0
    }
}

impl std::convert::From<WorkerID> for u64 {
    fn from(worker_id: WorkerID) -> u64 {
        worker_id.0 as u64
    }
}

impl PartialEq for WorkerID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Clone for WorkerID {
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
            (1 << WORKER_ID_BITS, false),
            ((1 << WORKER_ID_BITS) - 1, true),
        ];

        for (v, valid) in tests {
            match WorkerID::new(v) {
                Ok(worker_id) => {
                    assert!(valid);
                    assert_eq!(u32::from(worker_id), v);
                }
                Err(_) => assert!(!valid),
            }
        }
    }
}
