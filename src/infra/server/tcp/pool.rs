use super::*;

pub(crate) struct Pool {
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Message>,
}

impl Pool {
    pub fn new(size: usize) -> Result<Self, Error> {
        if size == 0 {
            return Err(Error::ThreadCreateFailed(String::from(
                "pool size need to be greater than 0",
            )));
        }

        let (tx, rx) = std::sync::mpsc::channel();
        let rx = std::sync::Arc::new(std::sync::Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, std::sync::Arc::clone(&rx)));
        }

        Ok(Pool {
            workers,
            sender: tx,
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
        log::debug!("sent a job");
    }
}
