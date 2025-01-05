use super::command;
use super::command_name::*;
use super::error::*;
use color_eyre::eyre::{Report, Result};

pub struct Parser();

impl Parser {
    pub fn parse(line: &str) -> Result<Box<dyn command::Command>> {
        let mut parts = line.trim().split_whitespace();
        let raw_command = parts.next().ok_or(Error::EmptyCommand)?;
        let command = CommandName::from_str(raw_command);
        let args = parts.map(|s| s.to_string()).collect();

        match command {
            CommandName::Get => Ok(Box::new(command::Get::new(args)?)),
            CommandName::Stat => todo!(),
            CommandName::Unknown(s) => Err(Error::UnknownCommand(s)).map_err(Report::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_success() {
        let tests = vec![("get key1", "GET key1"), ("get key1 key2", "GET key1 key2")];

        for (line, expected) in tests {
            let got = Parser::parse(line).unwrap().to_string();
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn test_parse_error() {
        let tests = vec![
            ("", Error::EmptyCommand),
            ("foo", Error::UnknownCommand("foo".to_string())),
        ];

        for (line, expected) in tests {
            match Parser::parse(line) {
                Ok(_) => panic!("expected error, but got Ok"),
                Err(e) => assert_eq!(e.to_string(), expected.to_string()),
            }
        }
    }
}
