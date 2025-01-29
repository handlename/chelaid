use color_eyre::eyre::{Report, Result};

use super::super::command_name::CommandName;
use super::super::error::Error;
use super::super::response;
use super::{Command, Response};
use crate::{app, domain};

pub struct Get {
    pub keys: Vec<String>,
    usecase: app::usecase::generate::Generate,
}

impl Get {
    pub fn new<R>(repository: std::sync::Arc<R>, keys: Vec<String>) -> Result<Self>
    where
        R: domain::repository::Id + 'static,
    {
        if keys.is_empty() {
            return Err(Error::InvalidArguments(format!(
                "{} command needs key(s)",
                String::from(CommandName::Get)
            )))
            .map_err(Report::from);
        }

        Ok(Self {
            keys,
            usecase: app::usecase::generate::Generate::new(repository),
        })
    }
}

impl Command for Get {
    fn execute(&self) -> Result<Vec<Box<dyn Response>>> {
        let mut results = Vec::with_capacity(self.keys.len());
        for key in &self.keys {
            let id = self.usecase.run()?;
            results.push(Box::new(response::Value::new(key, id)) as Box<dyn Response>);
        }

        Ok(results)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::repository::Id;
    use crate::domain::value_object;

    use super::*;

    #[derive(Clone)]
    struct MockRepository;

    impl domain::repository::Id for MockRepository {
        /// Returns a fixed ID for testing
        fn next(&self) -> Result<value_object::Id> {
            let ts = value_object::Timestamp::new(domain::TIMESTAMP_MIN).unwrap();
            let seq = value_object::Sequence::new(domain::SEQUENCE_MIN).unwrap();
            let worker_id = value_object::WorkerId::new(1).unwrap();
            Ok(domain::value_object::Id::new(ts, seq, worker_id))
        }
    }

    #[test]
    fn test_execute() {
        let repo = std::sync::Arc::new(MockRepository);
        let id = repo.next().unwrap();

        let tests = vec![
            (
                vec!["key1"],
                vec![format!("VALUE key1 0 {}\r\n", u64::from(id.clone()))],
            ),
            (
                vec!["key1", "key2"],
                vec![
                    format!("VALUE key1 0 {}\r\n", u64::from(id.clone())),
                    format!("VALUE key2 0 {}\r\n", u64::from(id.clone())),
                ],
            ),
        ];

        for (keys, expected) in tests {
            let keys = keys.iter().map(|s| s.to_string()).collect();
            let cmd = Get::new(repo.clone(), keys).unwrap();
            let got = cmd.execute().unwrap();
            let res = got.iter().map(|v| v.to_string()).collect::<Vec<String>>();
            assert_eq!(res, expected);
        }
    }
}
