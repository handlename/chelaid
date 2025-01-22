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

pub struct ShutdownInfo {
    flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
    addr: std::net::SocketAddr,
}

impl Clone for ShutdownInfo {
    fn clone(&self) -> Self {
        ShutdownInfo {
            flag: std::sync::Arc::clone(&self.flag),
            addr: self.addr,
        }
    }
}

pub struct Tcp<R>
where
    R: domain::repository::ID + Send + Sync + 'static,
{
    host: String,
    port: u16,
    repository: Option<R>,
}

impl<R> Tcp<R>
where
    R: domain::repository::ID + Send + Sync + 'static,
{
    pub fn new(host: String, port: u16, repository: R) -> Result<Self> {
        Ok(Self {
            host,
            port,
            repository: Some(repository),
        })
    }

    pub fn start(&mut self) -> Result<(std::thread::JoinHandle<()>, ShutdownInfo)> {
        let server = std::net::TcpListener::bind(format!("{}:{}", self.host, self.port))?;
        let local_addr = server.local_addr()?;
        log::info!("start TCP server");

        let parser = std::sync::Arc::new(infra::interface::memcached_text_basic::Parser::new(
            std::sync::Arc::new(self.repository.take().unwrap()),
        ));
        let pool = Pool::new(4)?; // TODO: make pool size configurable
        let should_run = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));

        let shutdown_info = ShutdownInfo {
            flag: should_run.clone(),
            addr: local_addr,
        };

        let handle = std::thread::spawn(move || {
            for connection in server.incoming() {
                if !should_run.load(std::sync::atomic::Ordering::SeqCst) {
                    return;
                }

                match connection {
                    Ok(stream) => {
                        let address = stream.peer_addr().unwrap();
                        log::debug!("accepted connection from {}", address);

                        let parser = std::sync::Arc::clone(&parser);
                        pool.execute(move || match handle_connection(stream, parser) {
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
        });

        Ok((handle, shutdown_info))
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

        // TODO: make timeout configurable
        match stream.set_read_timeout(Some(std::time::Duration::from_secs(5))) {
            Ok(_) => {}
            Err(_) => {
                log::error!("failed to set timeout for {}", address);
                break;
            }
        }

        match reader.read_line(&mut buf) {
            Ok(0) => {
                log::info!("connection closed by {}", address);
                break;
            }
            Ok(_) => {}
            Err(e) => {
                log::debug!("failed to read from {}: {}", address, e);
                break;
            }
        }

        match parser.parse(&buf) {
            Ok(command) => {
                if let Some(_) = command
                    .as_any()
                    .downcast_ref::<infra::interface::memcached_text_basic::command::End>(
                ) {
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
        }

        buf.clear();
    }

    stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}

pub fn shutdown(info: ShutdownInfo) -> Result<()> {
    info.flag.store(false, std::sync::atomic::Ordering::SeqCst);

    // for unlock blocking by incoming()
    std::net::TcpStream::connect(info.addr)?;

    Ok(())
}
