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

pub trait Publisher<R>: Actor<R>
where
    R: RuntimeHandle,
{
    type Event: Send + Unpin + 'static;

    fn register_subscriber(&mut self, sender: R::MpscSender<Self::Event>);
}

pub trait EventSubscription<R: RuntimeHandle> {
    type Event: Send + Unpin + 'static;

    fn message(sender: R::MpscSender<Self::Event>) -> Self
    where
        Self: Sized + Send;
}

impl<A, E, R> ActorHandle<A, R>
where
    A: Publisher<R, Event = E>,
    R: RuntimeHandle,
    A::Message: EventSubscription<R, Event = E>,
    E: Send,
{
    pub async fn subscribe(
        &mut self,
        sender: R::MpscSender<A::Event>,
    ) -> Result<(), <R::MpscSender<A::Message> as futures::Sink<A::Message>>::Error> {
        use futures::SinkExt;

        let message = A::Message::message(sender);

        self.sender.send(message).await
    }
}
