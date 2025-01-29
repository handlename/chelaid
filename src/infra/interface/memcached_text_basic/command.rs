mod end;
mod get;

use color_eyre::eyre::Result;

use super::response::Response;

pub use end::End;
pub use get::Get;

pub trait Command: std::any::Any {
    /// Execute the command and return the result.
    /// The result is a vector of strings.
    /// Each string represents a line of the response.
    fn execute(&self) -> Result<Vec<Box<dyn Response>>>;

    fn as_any(&self) -> &dyn std::any::Any;
}
