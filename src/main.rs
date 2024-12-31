use app::usecase;
use color_eyre::eyre::Result;

mod app;
mod domain;
mod infra;

fn main() -> Result<()> {
    color_eyre::install()?;
    let worker_id = domain::value_object::worker_id::WorkerID::new(0)?;
    let repo = infra::repository::id::ID::new(worker_id)?;
    let uc = usecase::generate::Generate::new(repo);
    let id = uc.run()?;
    println!("{:?}", id);
    Ok(())
}
