use crate::domain::constant::TIMESTAMP_OFFSET;

use super::super::constant::{SEQUENCE_BITS, WORKER_ID_BITS};
use super::sequence::Sequence;
use super::timestamp::Timestamp;
use super::worker_id::WorkerID;
use color_eyre::eyre::Result;

// ID structure:
// | timestamp | worker_id | sequence |
// | 41bit     | 10bit     | 12bit    |
#[derive(Debug, PartialOrd, Ord)]
pub struct ID(u64);

impl ID {
    pub fn new(ts: Timestamp, seq: Sequence, worker_id: WorkerID) -> ID {
        let v = ((u64::from(ts) - TIMESTAMP_OFFSET) << (WORKER_ID_BITS + SEQUENCE_BITS))
            | (u64::from(worker_id) << SEQUENCE_BITS)
            | u64::from(seq);

        ID(v)
    }

    pub fn parse(id: ID) -> Result<(Timestamp, Sequence, WorkerID)> {
        let v = id.0;

        let raw_ts = v >> (WORKER_ID_BITS + SEQUENCE_BITS);
        let raw_worker_id = (v >> SEQUENCE_BITS) & ((1 << WORKER_ID_BITS) - 1);
        let raw_seq = v & ((1 << SEQUENCE_BITS) - 1);

        let ts = Timestamp::new(raw_ts + TIMESTAMP_OFFSET)?;
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

impl PartialEq for ID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ID {}

impl Clone for ID {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ts = Timestamp::new(100000 + TIMESTAMP_OFFSET).unwrap();
        let seq = Sequence::new(1234).unwrap();
        let worker_id = WorkerID::new(567).unwrap();

        let id = ID::new(ts.clone(), seq.clone(), worker_id.clone());
        assert_eq!(
            u64::from(id.clone()),
            100000 << (WORKER_ID_BITS + SEQUENCE_BITS) | 567 << SEQUENCE_BITS | 1234
        );
        println!("{:?}", id);

        let (parsed_ts, parsed_seq, parsed_worker_id) = ID::parse(id).unwrap();
        assert_eq!(parsed_ts, ts);
        assert_eq!(parsed_seq, seq);
        assert_eq!(parsed_worker_id, worker_id);
    }
}
