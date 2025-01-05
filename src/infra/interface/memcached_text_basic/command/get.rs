use color_eyre::eyre::{Report, Result};

use super::super::error::Error;

use super::{super::command_name::CommandName, Command};

#[derive(Debug)]
pub struct Get {
    command: CommandName,
    keys: Vec<String>,
}

impl Get {
    pub fn new(keys: Vec<String>) -> Result<Self> {
        if keys.is_empty() {
            return Err(Error::InvalidArguments(format!(
                "{} command needs key(s)",
                String::from(CommandName::Get)
            )))
            .map_err(Report::from);
        }

        Ok(Self {
            command: CommandName::Get,
            keys,
        })
    }
}

impl Command for Get {
    fn execute(&self) -> Result<String> {
        todo!()
    }

    fn to_string(&self) -> String {
        format!(
            "{} {}",
            String::from(self.command.clone()),
            self.keys.join(" ")
        )
    }
}
