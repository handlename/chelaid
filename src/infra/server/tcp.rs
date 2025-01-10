mod error;
mod job;
mod message;
mod pool;
mod worker;

pub(crate) use error::*;
pub(crate) use job::*;
pub(crate) use message::*;
pub(crate) use pool::*;
pub(crate) use worker::*;

use std::io::{BufRead, Write};

use color_eyre::eyre::Result;
use log;

use crate::domain;
use crate::infra;

pub struct Tcp<R>
where
    R: domain::repository::ID + Send + Sync + 'static,
{
    host: String,
    port: u16,
    should_run: std::sync::Arc<std::sync::atomic::AtomicBool>,
    memcached_text_parser: std::sync::Arc<infra::interface::memcached_text_basic::Parser<R>>,
    pool: Pool,
}

impl<R> Tcp<R>
where
    R: domain::repository::ID + Send + Sync + 'static,
{
    pub fn new(host: String, port: u16, repository: R) -> Result<Self> {
        Ok(Self {
            host,
            port,
            should_run: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
            memcached_text_parser: std::sync::Arc::new(
                infra::interface::memcached_text_basic::Parser::new(std::sync::Arc::new(
                    repository,
                )),
            ),
            pool: Pool::new(4)?, // TODO: make it configurable
        })
    }

    pub fn start(&self) -> Result<()> {
        let server = std::net::TcpListener::bind(format!("{}:{}", self.host, self.port))?;
        // server.set_nonblocking(false).expect("out of service");
        log::info!("start TCP server");

        while self.should_run.load(std::sync::atomic::Ordering::SeqCst) {
            log::debug!("waiting connection");

            match server.accept() {
                Ok((stream, address)) => {
                    log::debug!("accepted connection from {}", address);

                    let parser = std::sync::Arc::clone(&self.memcached_text_parser);
                    self.pool
                        .execute(move || match handle_connection(stream, parser) {
                            Ok(_) => log::debug!("connection closed for {}", address),
                            Err(e) => {
                                log::error!("failed to handle connection for {}: {}", address, e)
                            }
                        });
                }
                Err(e) => {
                    log::error!("failed to read: {}", e);
                    break;
                }
            }
        }

        log::info!("server stopped");
        Ok(())
    }

    pub fn get_should_run(&self) -> std::sync::Arc<std::sync::atomic::AtomicBool> {
        self.should_run.clone()
    }
}

fn handle_connection<R>(
    mut stream: std::net::TcpStream,
    parser: std::sync::Arc<infra::interface::memcached_text_basic::Parser<R>>,
) -> Result<()>
where
    R: domain::repository::ID + Send + Sync + 'static,
{
    let address = stream.peer_addr()?;
    let mut reader = std::io::BufReader::new(stream.try_clone().unwrap());
    let mut buf = String::new();

    log::debug!("start waiting message loop from {}", address);

    loop {
        log::debug!("waiting message from {}", address);

        match reader.read_line(&mut buf) {
            Ok(0) => {
                log::info!("connection closed by {}", address);
                break;
            }
            Ok(_) => match parser.parse(&buf) {
                Ok(command) => {
                    if command.command_name()
                        == infra::interface::memcached_text_basic::CommandName::End
                    {
                        log::debug!("END command from {}", address);
                        break;
                    }

                    let res = command.execute()?;
                    for r in res {
                        log::debug!("response for {}: {}", address, r);

                        stream.write_all(r.as_bytes())?;
                        stream.write_all(b"\r\n")?;
                        stream.flush()?;
                    }
                }
                Err(e) => {
                    log::error!("failed to parse '{}' from {}: {}", buf, address, e);
                    // TODO: returns log::! to client
                    break;
                }
            },
            Err(e) => {
                log::debug!("failed to read from {}: {}", address, e);
                break;
            }
        }

        buf.clear();
    }

    stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}
