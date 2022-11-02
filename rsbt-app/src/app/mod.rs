use rsbt_rt::Runtime;

mod message_channel;

use crate::{Actor, ActorHandle};

pub use self::message_channel::MessageChannel;

const DEFAULT_CHANNEL_BUFFER: usize = 8;

#[derive(Default)]
pub struct App {}

impl App {
    pub fn message_channel<R: Runtime, T: Send + Unpin + 'static>(&self) -> MessageChannel<R, T> {
        MessageChannel::new(DEFAULT_CHANNEL_BUFFER)
    }

    pub fn start<A: Actor<R>, R: Runtime>(&self, actor: A) -> ActorHandle<A, R> {
        let (sender, receiver) = R::channel::<A::Message>(DEFAULT_CHANNEL_BUFFER);
        let _ = R::spawn_current(A::message_loop(actor, receiver));
        ActorHandle::new(sender)
    }
}
