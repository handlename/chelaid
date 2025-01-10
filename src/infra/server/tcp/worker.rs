use super::*;

pub(crate) struct Worker {
    pub id: usize,
    pub thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        id: usize,
        rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<Message>>>,
    ) -> Self {
        let thread = std::thread::spawn(move || loop {
            let message = {
                let rx = rx.lock().unwrap();
                log::debug!("worker {} waiting message", id);
                rx.recv().unwrap()
            };

            match message {
                Message::NewJob(job) => {
                    log::debug!("worker {} got a job", id);
                    job.call_box();
                }
                Message::Shutdown => todo!(),
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
