mod message_channel;

use crate::tokio::{mpsc_channel, spawn};

use crate::{Actor, ActorHandle, ActorMessageLoop, DEFAULT_CHANNEL_BUFFER};

pub use self::message_channel::MessageChannel;

#[derive(Default)]
pub struct App {}

impl App {
    pub fn message_channel<T>(&self) -> MessageChannel<T> {
        MessageChannel::new(DEFAULT_CHANNEL_BUFFER)
    }
}
