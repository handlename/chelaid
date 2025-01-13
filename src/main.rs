use clap::Parser;
use color_eyre::eyre::Result;
use log;

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

    /// Host name for TCP server
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Port number for TCP server
    #[arg(long, default_value_t = 11212)]
    port: u16,
}

fn main() -> Result<()> {
    env_logger::init();
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

    run_tcp_server(args)?;

    Ok(())
}

fn run_cli_issue(args: Args) -> Result<()> {
    log::debug!("start to issue a id...");

    let worker_id = domain::value_object::WorkerID::new(args.worker_id)?;
    let repository = infra::repository::ID::new(worker_id)?;

    let id = infra::interface::Cli::new(repository).issue()?;
    println!("{}", u64::from(id));

    Ok(())
}

fn run_cli_issue_some(args: Args) -> Result<()> {
    log::debug!("start to issue some ids...");

    let worker_id = domain::value_object::WorkerID::new(args.worker_id)?;
    let repository = infra::repository::ID::new(worker_id)?;
    let num = args.issues.unwrap();

    let ids = infra::interface::Cli::new(repository).issue_some(num)?;
    for id in ids {
        println!("{}", u64::from(id));
    }

    Ok(())
}

fn run_tcp_server(args: Args) -> Result<()> {
    log::debug!("start to run TCP server...");

    let worker_id = domain::value_object::WorkerID::new(args.worker_id)?;
    let repository = infra::repository::ID::new(worker_id)?;

    let mut server = infra::server::tcp::Tcp::new(args.host, args.port, repository)?;

    let (handle, shutdown_info) = server.start()?;

    ctrlc::set_handler(move || {
        log::info!("received Ctrl+C, shutting down...");
        match  infra::server::tcp::shutdown(shutdown_info.clone()) {
            Ok(_) => log::info!("server shutdown successfully"),
            Err(e) => log::error!("failed to shutdown server: {}", e),
        }
    })
    .expect("error on setting Ctrl+C handler");

    handle.join().expect("failed to join server thread");

    Ok(())
}
