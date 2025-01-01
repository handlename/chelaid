use clap::Parser;
use color_eyre::eyre::Result;

mod app;
mod domain;
mod infra;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Worker ID
    #[arg(short, long)]
    worker_id: u32,

    /// Issue a ID
    #[arg(long, default_value_t = false)]
    issue: bool,

    /// Number of IDs to issue
    #[arg(long)]
    issues: Option<usize>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    if args.issue {
        run_cli_issue(args)?;
        return Ok(());
    }

    if let Some(_) = args.issues {
        run_cli_issue_some(args)?;
        return Ok(());
    }

    Ok(())
}

fn run_cli_issue(args: Args) -> Result<()> {
    let worker_id = domain::value_object::worker_id::WorkerID::new(args.worker_id)?;
    let repository = infra::repository::id::ID::new(worker_id)?;

    let id = infra::interface::cli::Cli::new(repository).issue()?;
    println!("{}", u64::from(id));

    Ok(())
}

fn run_cli_issue_some(args: Args) -> Result<()> {
    let worker_id = domain::value_object::worker_id::WorkerID::new(args.worker_id)?;
    let repository = infra::repository::id::ID::new(worker_id)?;
    let num = args.issues.unwrap();

    let ids = infra::interface::cli::Cli::new(repository).issue_some(num)?;
    for id in ids {
        println!("{}", u64::from(id));
    }

    Ok(())
}
