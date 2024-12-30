use super::super::constant::{SEQUENCE_BITS, WORKER_ID_BITS};
use super::sequence::Sequence;
use super::worker_id::WorkerID;
use color_eyre::eyre::Result;
use std::time::SystemTime;

// ID structure:
// | timestamp | worker_id | sequence |
// | 41bit     | 10bit     | 12bit    |
pub struct ID(u64);

impl ID {
    pub fn new(ts: SystemTime, seq: Sequence, worker_id: WorkerID) -> Result<ID> {
        let epoch = ts.elapsed().unwrap().as_secs();

        let v = (epoch << (WORKER_ID_BITS + SEQUENCE_BITS))
            | (u64::from(worker_id) << SEQUENCE_BITS)
            | u64::from(seq);

        Ok(ID(v))
    }

    pub fn parse(id: ID) -> Result<(SystemTime, Sequence, WorkerID)> {
        let v = id.0;

        let epoch = v >> (WORKER_ID_BITS + SEQUENCE_BITS);
        let raw_worker_id = (v >> SEQUENCE_BITS) & ((1 << WORKER_ID_BITS) - 1);
        let raw_seq = v & ((1 << SEQUENCE_BITS) - 1);

        let ts = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(epoch);
        let worker_id = WorkerID::new(raw_worker_id as u32)?;
        let seq = Sequence::new(raw_seq as u32)?;

        Ok((ts, seq, worker_id))
    }
}

impl std::convert::From<ID> for u64 {
    fn from(id: ID) -> u64 {
        id.0
    }
}
