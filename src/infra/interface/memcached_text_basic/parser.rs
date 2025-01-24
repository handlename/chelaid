use crate::domain;

use super::command;
use super::command_name::*;
use super::error::*;
use color_eyre::eyre::{Report, Result};

pub struct Parser<R>
where
    R: domain::repository::Id + Send + Sync + 'static,
{
    repository: std::sync::Arc<R>,
}

impl<R> Parser<R>
where
    R: domain::repository::Id + Send + Sync + 'static,
{
    pub fn new(repository: std::sync::Arc<R>) -> Self {
        Self { repository }
    }

    pub fn parse(&self, line: &str) -> Result<Box<dyn command::Command>> {
        let mut parts = line.trim().split_whitespace();
        let raw_command = parts.next().ok_or(Error::EmptyCommand)?;
        let command = CommandName::from_str(raw_command);
        let args = parts.map(|s| s.to_string()).collect();

        match command {
            CommandName::Get => Ok(Box::new(command::Get::new(self.repository.clone(), args)?)),
            CommandName::Stat => todo!(),
            CommandName::End => Ok(Box::new(command::End::new())),
            CommandName::Unknown(s) => Err(Error::UnknownCommand(s)).map_err(Report::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct MockRepository;

    impl domain::repository::Id for MockRepository {
        fn next(&self) -> Result<domain::value_object::Id> {
            let ts = domain::value_object::Timestamp::new(domain::TIMESTAMP_OFFSET).unwrap();
            let seq = domain::value_object::Sequence::new(1).unwrap();
            let worker_id = domain::value_object::WorkerId::new(1).unwrap();
            Ok(domain::value_object::Id::new(ts, seq, worker_id))
        }
    }

    #[test]
    fn test_parse_success() {
            let tests = vec![
                ("get key1", vec!["key1"]),
                ("get key1 key2", vec!["key1", "key2"]),
                ("GET key1", vec!["key1"]),
                ("Get key1", vec!["key1"]),
            ];

            let repo = std::sync::Arc::new(MockRepository);
            let parser = Parser::new(repo);

            for (line, expected) in tests {
                let res = parser.parse(line).expect("unexpected parse error");
                if let Some(command) = res.as_any().downcast_ref::<command::Get>() {
                    assert_eq!(command.keys, expected.iter().map(|s| s.to_string()).collect::<Vec<_>>());
                } else {
                    panic!("unexpected command type");
                }
            }
        }

    #[test]
    fn test_parse_error() {
        let tests = vec![
            ("", Error::EmptyCommand),
            ("foo", Error::UnknownCommand("foo".to_string())),
        ];

        let repo = MockRepository;
        let parser = Parser::new(std::sync::Arc::new(repo));

        for (line, expected) in tests {
            match parser.parse(line) {
                Ok(_) => panic!("expected error, but got Ok"),
                Err(e) => assert_eq!(e.to_string(), expected.to_string()),
            }
        }
    }
}
