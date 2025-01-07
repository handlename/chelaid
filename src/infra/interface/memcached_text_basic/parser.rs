use crate::domain;

use super::command;
use super::command_name::*;
use super::error::*;
use color_eyre::eyre::{Report, Result};

pub struct Parser<R>
where
    R: domain::repository::ID,
{
    repository: R,
}

impl<R> Parser<R>
where
    R: domain::repository::ID + Clone + Copy + 'static,
{
    pub fn new(repository: R) -> Self {
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
            CommandName::Unknown(s) => Err(Error::UnknownCommand(s)).map_err(Report::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct MockRepository;

    impl domain::repository::ID for MockRepository {
        fn next(&self) -> Result<domain::value_object::ID> {
            let ts = domain::value_object::Timestamp::new(domain::TIMESTAMP_OFFSET).unwrap();
            let seq = domain::value_object::Sequence::new(1).unwrap();
            let worker_id = domain::value_object::WorkerID::new(1).unwrap();
            Ok(domain::value_object::ID::new(ts, seq, worker_id))
        }
    }

    #[test]
    fn test_parse_success() {
        let tests = vec![
            ("get key1", "GET key1"),
            ("get key1 key2", "GET key1 key2"),
            ("GET key1", "GET key1"),
            ("Get key1", "GET key1"),
        ];

        let repo = MockRepository;
        let parser = Parser::new(repo);

        for (line, expected) in tests {
            let got = parser.parse(line).unwrap().to_string();
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn test_parse_error() {
        let tests = vec![
            ("", Error::EmptyCommand),
            ("foo", Error::UnknownCommand("foo".to_string())),
        ];

        let repo = MockRepository;
        let parser = Parser::new(repo);

        for (line, expected) in tests {
            match parser.parse(line) {
                Ok(_) => panic!("expected error, but got Ok"),
                Err(e) => assert_eq!(e.to_string(), expected.to_string()),
            }
        }
    }
}
