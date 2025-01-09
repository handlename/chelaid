use std::io::{BufRead, Write};

use color_eyre::eyre::Result;
use log::{debug, error, info};

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
        })
    }

    pub fn start(&self) -> Result<()> {
        let server = std::net::TcpListener::bind(format!("{}:{}", self.host, self.port))?;
        server.set_nonblocking(false).expect("out of service");
        info!("start TCP server");

        while self.should_run.load(std::sync::atomic::Ordering::SeqCst) {
            debug!("waiting connection");

            match server.accept() {
                Ok((stream, address)) => match stream.try_clone() {
                    Ok(mut s) => {
                        debug!("accepted connection from {}", address);
                        self.handle_connection(&mut s)?;
                    }
                    Err(e) => {
                        error!("failed to clone stream from address {}: {}", address, e);
                    }
                },
                Err(e) => {
                    error!("failed to read: {}", e);
                    break;
                }
            }
        }

        info!("server stopped");
        Ok(())
    }

    pub fn get_should_run(&self) -> std::sync::Arc<std::sync::atomic::AtomicBool> {
        self.should_run.clone()
    }

    fn handle_connection(&self, stream: &mut std::net::TcpStream) -> Result<()> {
        let address = stream.peer_addr()?;
        let mut reader = std::io::BufReader::new(stream.try_clone().unwrap());
        let mut buf = String::new();

        debug!("start waiting message loop from {}", address);

        loop {
            debug!("waiting message from {}", address);

            match reader.read_line(&mut buf) {
                Ok(0) => {
                    info!("connection closed by {}", address);
                    break;
                }
                Ok(_) => match self.memcached_text_parser.parse(&buf) {
                    Ok(command) => {
                        if command.command_name()
                            == infra::interface::memcached_text_basic::CommandName::End
                        {
                            debug!("END command from {}", address);
                            return Ok(());
                        }

                        let res = command.execute()?;
                        for r in res {
                            debug!("response for {}: {}", address, r);

                            stream.write(r.as_bytes())?;
                            stream.write_all(b"\r\n")?;
                        }
                    }
                    Err(e) => {
                        error!("failed to parse '{}' from {}: {}", buf, address, e);
                        break;
                    }
                },
                Err(e) => {
                    error!("failed to read from {}: {}", address, e);
                    break;
                }
            }

            buf.clear();
        }

        Ok(())
    }
}
