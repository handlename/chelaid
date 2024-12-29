use super::sequence::Sequence;
use super::worker_id::WorkerID;
use color_eyre::eyre::Result;
use std::time::SystemTime;

// ID structure:
// | timestamp | worker_id | sequence |
// | 41bit     | 10bit     | 12bit    |
pub type ID = u64;

const WORKER_ID_BITS: i32 = 10;
const SEQUENCE_BITS: i32 = 12;

pub fn build(ts: SystemTime, seq: Sequence, worker_id: WorkerID) -> Result<ID> {
    let epoch = ts.elapsed().unwrap().as_secs();

    Ok((epoch << (WORKER_ID_BITS + SEQUENCE_BITS))
        | ((worker_id as u64) << SEQUENCE_BITS)
        | seq as u64)
}

pub fn parse(id: ID) -> Result<(SystemTime, Sequence, WorkerID)> {
    let epoch = id >> (WORKER_ID_BITS + SEQUENCE_BITS);
    let worker_id = (id >> SEQUENCE_BITS) & ((1 << WORKER_ID_BITS) - 1);
    let seq = id & ((1 << SEQUENCE_BITS) - 1);

    let ts = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(epoch);

    Ok((ts, seq as Sequence, worker_id as WorkerID))
}
