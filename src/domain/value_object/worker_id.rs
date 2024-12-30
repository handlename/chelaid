use color_eyre::eyre::{eyre, Result};

use crate::domain::constant::WORKER_ID_BITS;

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
