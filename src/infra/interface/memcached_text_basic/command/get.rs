use color_eyre::eyre::{Report, Result};

use super::super::error::Error;
use super::{super::command_name::CommandName, Command};
use crate::{app, domain};

pub struct Get {
    command_name: CommandName,
    keys: Vec<String>,
    usecase: app::usecase::generate::Generate,
}

impl Get {
    pub fn new<R>(repository: std::sync::Arc<R>, keys: Vec<String>) -> Result<Self>
    where
        R: domain::repository::ID + 'static,
    {
        if keys.is_empty() {
            return Err(Error::InvalidArguments(format!(
                "{} command needs key(s)",
                String::from(CommandName::Get)
            )))
            .map_err(Report::from);
        }

        Ok(Self {
            command_name: CommandName::Get,
            keys,
            usecase: app::usecase::generate::Generate::new(repository),
        })
    }
}

impl Command for Get {
    fn execute(&self) -> Result<Vec<String>> {
        let mut results = Vec::with_capacity(self.keys.len());
        for key in &self.keys {
            let id = self.usecase.run()?;
            results.push(format!("VALUE {} 0 {}", key, u64::from(id)))
        }

        Ok(results)
    }

    fn command_name(&self) -> CommandName {
        self.command_name.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "{} {}",
            String::from(self.command_name.clone()),
            self.keys.join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::repository::ID;
    use crate::domain::value_object;

    use super::*;

    #[derive(Clone)]
    struct MockRepository;

    impl domain::repository::ID for MockRepository {
        fn next(&self) -> Result<value_object::ID> {
            let ts = value_object::Timestamp::new(domain::TIMESTAMP_MIN).unwrap();
            let seq = value_object::Sequence::new(domain::SEQUENCE_MIN).unwrap();
            let worker_id = value_object::WorkerID::new(1).unwrap();
            Ok(domain::value_object::ID::new(ts, seq, worker_id))
        }
    }

    #[test]
    fn test_execute() {
        let repo = std::sync::Arc::new(MockRepository);
        let id = repo.next().unwrap();

        let tests = vec![
            (
                vec!["key1"],
                vec![format!("VALUE key1 0 {}", u64::from(id.clone()))],
            ),
            (
                vec!["key1", "key2"],
                vec![
                    format!("VALUE key1 0 {}", u64::from(id.clone())),
                    format!("VALUE key2 0 {}", u64::from(id.clone())),
                ],
            ),
        ];

        for (keys, expected) in tests {
            let keys = keys.iter().map(|s| s.to_string()).collect();
            let cmd = Get::new(repo.clone(), keys).unwrap();
            let got = cmd.execute().unwrap();
            assert_eq!(got, expected);
        }
    }
}
