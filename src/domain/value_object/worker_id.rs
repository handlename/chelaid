use super::super::constant::WORKER_ID_BITS;
use super::super::error::Error;

use color_eyre::eyre::Result;

#[derive(Debug)]
pub struct WorkerId(u32);

impl WorkerId {
    pub fn new(v: u32) -> Result<WorkerId, Error> {
        if (1 << WORKER_ID_BITS) <= v {
            return Err(Error::WorkerIDTooLarge(v));
        }

        Ok(WorkerId(v))
    }
}

impl std::convert::From<WorkerId> for u32 {
    fn from(worker_id: WorkerId) -> u32 {
        worker_id.0
    }
}

impl std::convert::From<WorkerId> for u64 {
    fn from(worker_id: WorkerId) -> u64 {
        worker_id.0 as u64
    }
}

impl PartialEq for WorkerId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Clone for WorkerId {
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
            match WorkerId::new(v) {
                Ok(worker_id) => {
                    assert!(valid);
                    assert_eq!(u32::from(worker_id), v);
                }
                Err(_) => assert!(!valid),
            }
        }
    }
}
