mod value;
pub use value::Value;

pub trait Response: Into<String> {}
