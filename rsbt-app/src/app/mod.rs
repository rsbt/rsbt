mod message_channel;

use crate::tokio::{mpsc_channel, spawn};

use crate::{Actor, ActorHandle, ActorMessageLoop};

pub use self::message_channel::MessageChannel;

const DEFAULT_CHANNEL_BUFFER: usize = 8;

#[derive(Default)]
pub struct App {}

impl App {
    pub fn message_channel<T>(&self) -> MessageChannel<T> {
        MessageChannel::new(DEFAULT_CHANNEL_BUFFER)
    }

    pub async fn start<A>(&self, actor: A) -> ActorHandle<A>
    where
        A: Actor + Send + 'static,
        A::Message: Send,
    {
        let (sender, receiver) = mpsc_channel(DEFAULT_CHANNEL_BUFFER);
        let _ = spawn(ActorMessageLoop::message_loop(actor, receiver));
        ActorHandle::new(sender)
    }
}
