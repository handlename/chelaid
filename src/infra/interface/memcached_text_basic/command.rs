mod end;
mod get;

use color_eyre::eyre::Result;
pub use end::End;
pub use get::Get;

pub trait Command {
    /// Execute the command and return the result.
    /// The result is a vector of strings.
    /// Each string represents a line of the response.
    fn execute(&self) -> Result<Vec<String>>;

    /// Return the command name.
    fn command_name(&self) -> super::command_name::CommandName;

    /// Convert the command to a string.
    /// format: "<command> <arg1> <arg2> ..."
    fn to_string(&self) -> String;
}
