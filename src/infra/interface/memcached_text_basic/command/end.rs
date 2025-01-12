use color_eyre::eyre::Result;

use super::Command;

pub struct End {}

impl End {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for End {
    fn execute(&self) -> Result<Vec<String>> {
        // do nothing
        Ok(Vec::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
