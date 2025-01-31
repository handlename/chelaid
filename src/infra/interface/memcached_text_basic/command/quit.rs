use color_eyre::eyre::Result;

use super::Command;
use super::Response;

pub struct Quit {}

impl Quit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Quit {
    fn execute(&self) -> Result<Vec<Box<dyn Response>>> {
        // do nothing
        Ok(Vec::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
