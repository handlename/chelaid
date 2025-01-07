mod get;

use color_eyre::eyre::Result;
pub use get::Get;

pub trait Command {
    /// Execute the command and return the result.
    /// The result is a vector of strings.
    /// Each string represents a line of the response.
    fn execute(&self) -> Result<Vec<String>>;

    /// Convert the command to a string.
    /// format: "<command> <arg1> <arg2> ..."
    fn to_string(&self) -> String;
}
