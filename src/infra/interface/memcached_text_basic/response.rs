mod error;
mod value;
pub use value::Value;

pub trait Response {
    fn to_string(&self) -> String;
}
