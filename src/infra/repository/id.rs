use super::super::error::Error;
use crate::{
    domain::{self, value_object},
    infra::error,
};

use color_eyre::eyre::{Report, Result};

pub struct Id {
    worker_id: value_object::WorkerId,
    last: std::sync::Mutex<(value_object::Timestamp, value_object::Sequence)>,
}

impl Id {
    pub fn new(worker_id: value_object::WorkerId) -> Result<Self> {
        let last_ts = value_object::Timestamp::new(domain::TIMESTAMP_OFFSET)?;
        let last_seq = value_object::Sequence::new(0)?;
        let last = std::sync::Mutex::new((last_ts, last_seq));

        Ok(Self { worker_id, last })
    }

    fn forward_last(&self) -> Result<(value_object::Timestamp, value_object::Sequence)> {
        let mut last = self.last.lock().unwrap();

        let last_ts = &mut last.0.clone();
        let last_seq = &mut last.1.clone();

        let next_ts = value_object::Timestamp::new_from_system_time(std::time::SystemTime::now())?;
        if next_ts < *last_ts {
            return Err(Error::SystemClockRewound).map_err(Report::from);
        }

        // forward sequence if the timestamp is the same
        // reset sequence if the timestamp is different
        // reset sequence if the sequence is overflowed
        let (next_ts, next_seq) = if next_ts == *last_ts {
            match last_seq.clone().next() {
                Ok(seq) => (next_ts, seq),
                Err(err) => match err {
                    domain::Error::SequenceTooLarge(_) => {
                        let new_ts = self.wait_until_next_tick(next_ts.clone())?;
                        let new_seq = value_object::Sequence::new(0)?;
                        (new_ts, new_seq)
                    }
                    _ => return Err(error::Error::DomainError(err)).map_err(Report::from),
                },
            }
        } else {
            (next_ts, value_object::Sequence::new(0)?)
        };

        // update last state
        *last = (next_ts.clone(), next_seq.clone());

        Ok((next_ts, next_seq))
    }

    fn wait_until_next_tick(
        &self,
        current_ts: value_object::Timestamp,
    ) -> Result<value_object::Timestamp> {
        loop {
            let next_ts = value_object::Timestamp::now()?;

            if current_ts < next_ts {
                return Ok(next_ts);
            }

            std::thread::sleep(std::time::Duration::from_nanos(50));
        }
    }
}

impl crate::domain::repository::Id for Id {
    fn next(&self) -> Result<value_object::Id> {
        let (next_ts, next_seq) = self.forward_last()?;

        Ok(value_object::Id::new(
            next_ts,
            next_seq,
            self.worker_id.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::repository::Id;

    use super::*;

    #[test]
    fn test_next() {
        let worker_id = value_object::WorkerId::random();
        let repo = super::Id::new(worker_id.clone()).unwrap();
        let id = repo.next().unwrap();
        println!("generated id = {:?}", id);

        let (_, _, parsed_worker_id) = value_object::Id::parse(id).unwrap();
        assert_eq!(parsed_worker_id, worker_id);
    }

    #[test]
    fn test_next_some() {
        let worker_id = value_object::WorkerId::random();
        let repo = super::Id::new(worker_id.clone()).unwrap();

        let num = 10000;
        assert!(
            (1 << domain::SEQUENCE_BITS) < num,
            "num must be larger than 2^SEQUENCE_BITS for testing sequence overflow"
        );

        // minimum ID
        let mut last_id = value_object::Id::new(
            value_object::Timestamp::new(domain::TIMESTAMP_OFFSET).unwrap(),
            value_object::Sequence::new(0).unwrap(),
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
