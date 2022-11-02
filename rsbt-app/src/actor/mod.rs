use std::{future::Future, marker::PhantomData, pin::Pin};

use rsbt_rt::RuntimeHandle;

mod download;

pub use download::{Download, DownloadEvent, DownloadHandle};

pub trait Actor<R: RuntimeHandle> {
    type Message: Send + Unpin + 'static;

    fn handle_message(&mut self, msg: Self::Message);

    fn message_loop(
        actor: Self,
        receiver: R::MpscReceiver<Self::Message>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}

pub struct ActorHandle<A: Actor<R>, R: RuntimeHandle> {
    sender: R::MpscSender<A::Message>,
    _actor: PhantomData<A>,
}

impl<A: Actor<R>, R: RuntimeHandle> ActorHandle<A, R> {
    pub fn new(sender: R::MpscSender<A::Message>) -> Self {
        Self {
            sender,
            _actor: PhantomData,
        }
    }
}
