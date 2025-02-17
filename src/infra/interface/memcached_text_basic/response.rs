mod end;
mod error;
mod value;
pub use end::End;
pub use value::Value;

pub trait Response {
    fn to_string(&self) -> String;
}
