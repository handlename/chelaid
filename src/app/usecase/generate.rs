use crate::domain::{self, value_object};
use color_eyre::eyre::Result;

pub struct Generate {
    repository: Box<dyn domain::repository::ID>,
}

impl Generate {
    pub fn new<T>(repository: T) -> Generate
    where
        T: domain::repository::ID + 'static,
    {
        Self {
            repository: Box::new(repository),
        }
    }

    pub fn run(&self) -> Result<value_object::ID> {
        self.repository.next()
    }
}
