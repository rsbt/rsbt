mod message_channel;

use crate::tokio::{mpsc_channel, spawn};

use crate::{Actor, ActorHandle};

pub use self::message_channel::MessageChannel;

const DEFAULT_CHANNEL_BUFFER: usize = 8;

#[derive(Default)]
pub struct App {}

impl App {
    pub fn message_channel<T: Send + Unpin + 'static>(&self) -> MessageChannel<T> {
        MessageChannel::new(DEFAULT_CHANNEL_BUFFER)
    }

    pub async fn start<A: Actor>(&self, actor: A) -> ActorHandle<A> {
        let (sender, receiver) = mpsc_channel::<A::Message>(DEFAULT_CHANNEL_BUFFER);
        let _ = spawn(A::message_loop(actor, receiver));
        ActorHandle::new(sender)
    }
}
