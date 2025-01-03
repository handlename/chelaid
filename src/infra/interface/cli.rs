use crate::{
    app,
    domain::{self, value_object},
};
use color_eyre::eyre::Result;

pub struct Cli {
    usecase: app::usecase::generate::Generate,
}

impl Cli {
    pub fn new<R>(repository: R) -> Self
    where
        R: domain::repository::ID + 'static,
    {
        Self {
            usecase: app::usecase::generate::Generate::new(repository),
        }
    }

    pub fn issue(&self) -> Result<value_object::ID> {
        self.usecase.run()
    }

    pub fn issue_some(&self, n: usize) -> Result<Vec<value_object::ID>> {
        let mut ids = Vec::with_capacity(n);
        for _ in 0..n {
            ids.push(self.usecase.run()?);
        }
        Ok(ids)
    }
}
