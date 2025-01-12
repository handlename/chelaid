mod end;
mod get;

use color_eyre::eyre::Result;
pub use end::End;
pub use get::Get;

pub trait Command: std::any::Any {
    /// Execute the command and return the result.
    /// The result is a vector of strings.
    /// Each string represents a line of the response.
    fn execute(&self) -> Result<Vec<String>>;

    fn as_any(&self) -> &dyn std::any::Any;
}
