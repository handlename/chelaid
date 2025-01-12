use color_eyre::eyre::Result;

use crate::infra::interface::memcached_text_basic::command_name::CommandName;

use super::Command;

pub struct End {
    command_name: CommandName,
}

impl End {
    pub fn new() -> Self {
        Self {
            command_name: CommandName::End,
        }
    }
}

impl Command for End {
    fn execute(&self) -> Result<Vec<String>> {
        // do nothing
        Ok(Vec::new())
    }

    fn command_name(&self) -> CommandName {
        self.command_name.clone()
    }

    fn to_string(&self) -> String {
        String::from(self.command_name.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
