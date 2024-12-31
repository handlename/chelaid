use crate::domain::{self, value_object::id::ID};
use color_eyre::eyre::Result;

pub struct Generate {
    repository: Box<dyn domain::repository::id::ID>,
}

impl Generate {
    pub fn new<T>(repository: T) -> Generate
    where
        T: domain::repository::id::ID + 'static,
    {
        Self {
            repository: Box::new(repository),
        }
    }

    pub fn run(&self) -> Result<ID> {
        self.repository.next()
    }
}
