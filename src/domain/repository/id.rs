use super::super::value_object;
use color_eyre::eyre::Result;

pub trait ID {
    fn next(&self) -> Result<value_object::id::ID>;
}
