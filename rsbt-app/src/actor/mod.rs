use std::{future::Future, pin::Pin};

use crate::{
    tokio::{MpscReceiver, MpscSender},
    AppError,
};

mod download;

pub use download::{Download, DownloadEvent, DownloadHandle};

pub trait Actor {
    type Message: Send + Unpin + 'static;

    fn handle_message(&mut self, msg: Self::Message);

    fn message_loop(
        actor: Self,
        receiver: MpscReceiver<Self::Message>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}

pub struct ActorHandle<A: Actor> {
    sender: MpscSender<A::Message>,
}

impl<A: Actor> ActorHandle<A> {
    pub fn new(sender: MpscSender<A::Message>) -> Self {
        Self { sender }
    }
}

pub trait Publisher: Actor {
    type Event: Send + Unpin + 'static;

    fn register_subscriber(&mut self, sender: MpscSender<Self::Event>);
}

pub trait EventSubscription {
    type Event: Send + Unpin + 'static;

    fn message(sender: MpscSender<Self::Event>) -> Self
    where
        Self: Sized + Send;
}

impl<A, E> ActorHandle<A>
where
    A: Publisher<Event = E>,
    A::Message: EventSubscription<Event = E>,
    E: Send,
{
    pub async fn subscribe(&mut self, sender: MpscSender<A::Event>) -> Result<(), AppError> {
        let message = A::Message::message(sender);

        self.sender
            .send(message)
            .await
            .map_err(|_| AppError::Unknown) // TODO: error handling
    }
}
