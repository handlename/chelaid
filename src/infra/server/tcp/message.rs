use super::*;

pub(crate) enum Message {
    NewJob(Job),
    Shutdown,
}
