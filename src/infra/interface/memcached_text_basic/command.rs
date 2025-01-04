use color_eyre::eyre::Result;

pub trait Command {
    fn execute(&self) -> Result<String>;
}
