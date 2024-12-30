use crate::domain::{constant::TIMESTAMP_OFFSET, value_object};
use color_eyre::eyre::Result;

pub struct ID {
    worker_id: value_object::worker_id::WorkerID,
    last: std::sync::Mutex<(
        value_object::timestamp::Timestamp,
        value_object::sequence::Sequence,
    )>,
}

impl ID {
    pub fn new(worker_id: value_object::worker_id::WorkerID) -> Result<Self> {
        let last_ts = value_object::timestamp::Timestamp::new(TIMESTAMP_OFFSET)?;
        let last_seq = value_object::sequence::Sequence::new(0)?;
        let last = std::sync::Mutex::new((last_ts, last_seq));

        Ok(Self { worker_id, last })
    }

    fn forward_last(
        &self,
    ) -> Result<(
        value_object::timestamp::Timestamp,
        value_object::sequence::Sequence,
    )> {
        let mut last = self
            .last
            .lock()
            .map_err(|e| color_eyre::eyre::eyre!("Failed to lock last mutex: {}", e))?;

        let last_ts = &mut last.0.clone();
        let last_seq = &mut last.1.clone();

        let next_ts =
            value_object::timestamp::Timestamp::new_from_system_time(std::time::SystemTime::now())?;
        if next_ts < *last_ts {
            return Err(color_eyre::eyre::eyre!("system clock has been rollbacked"));
        }

        // TODO: Implement sequence overflow
        let next_seq = value_object::sequence::Sequence::new(u32::from((*last_seq).clone()) + 1)?;

        *last = (next_ts.clone(), next_seq.clone());

        Ok((next_ts, next_seq))
    }
}

impl crate::domain::repository::id::ID for ID {
    fn next(&self) -> Result<value_object::id::ID> {
        let (next_ts, next_seq) = self.forward_last()?;

        Ok(value_object::id::ID::new(
            next_ts,
            next_seq,
            self.worker_id.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{constant::SEQUENCE_BITS, repository::id::ID};

    use super::*;

    #[test]
    fn test_next() {
        let worker_id = value_object::worker_id::WorkerID::new(123).unwrap();
        let repo = super::ID::new(worker_id.clone()).unwrap();
        let id = repo.next().unwrap();
        println!("generated id = {:?}", id);

        let (_, _, parsed_worker_id) = value_object::id::ID::parse(id).unwrap();
        assert_eq!(parsed_worker_id, worker_id);
    }

    #[test]
    #[ignore] // sequence overflow is not implemented yet
    fn test_next_some() {
        let worker_id = value_object::worker_id::WorkerID::new(123).unwrap();
        let repo = super::ID::new(worker_id.clone()).unwrap();

        let num = 10000;
        assert!(SEQUENCE_BITS < num);

        // minimum ID
        let mut last_id = value_object::id::ID::new(
            value_object::timestamp::Timestamp::new(TIMESTAMP_OFFSET).unwrap(),
            value_object::sequence::Sequence::new(0).unwrap(),
            worker_id.clone(),
        );

        for _ in 0..num {
            let id = repo.next().unwrap();
            println!("generated id = {:?}", id);

            assert!(last_id < id);
            last_id = id;
        }
    }
}
