mod get;

use color_eyre::eyre::Result;
pub use get::Get;

pub trait Command {
    fn execute(&self) -> Result<String>;

    /// Convert the command to a string.
    /// format: "<command> <arg1> <arg2> ..."
    fn to_string(&self) -> String;
}
