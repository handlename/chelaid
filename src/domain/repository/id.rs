use crate::domain::value_object;
use color_eyre::eyre::Result;

pub trait ID {
    fn next() -> Result<value_object::id::ID>;
}
