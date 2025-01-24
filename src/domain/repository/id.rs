use super::super::value_object;
use color_eyre::eyre::Result;

pub trait Id {
    fn next(&self) -> Result<value_object::Id>;
}
