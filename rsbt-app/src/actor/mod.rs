use std::{future::Future, marker::PhantomData, pin::Pin};

use crate::{
    tokio::{MpscReceiver, MpscSender},
    AppError,
};

mod download;

pub use download::{Download, DownloadHandle, DownloadMessage};

pub trait Actor {
    type Message;

    fn handle_message(&mut self, msg: Self::Message);
}

#[derive(Default)]
pub(crate) struct ActorMessageLoop<A: Actor>(PhantomData<A>);

impl<A> ActorMessageLoop<A>
where
    A: Actor,
{
    pub async fn message_loop(mut actor: A, mut receiver: MpscReceiver<ActorCommand<A::Message>>) {
        while let Some(message) = receiver.recv().await {
            match message {
                ActorCommand::Message(message) => actor.handle_message(message),
            }
        }
    }
}

pub struct ActorHandle<A: Actor> {
    sender: MpscSender<ActorCommand<A::Message>>,
}

impl<A: Actor> ActorHandle<A> {
    pub fn new(sender: MpscSender<ActorCommand<A::Message>>) -> Self {
        Self { sender }
    }
}

pub trait Publisher: Actor {
    type Event;

    fn register_subscriber(&mut self, sender: MpscSender<Self::Event>);
}

pub trait EventSubscription {
    type Event;

    fn message(sender: MpscSender<Self::Event>) -> Self
    where
        Self: Sized;
}

impl<A, E> ActorHandle<A>
where
    A: Publisher<Event = E>,
    A::Message: EventSubscription<Event = E>,
{
    pub async fn subscribe(&mut self, sender: MpscSender<A::Event>) -> Result<(), AppError> {
        let message = ActorCommand::Message(A::Message::message(sender));

        self.sender
            .send(message)
            .await
            .map_err(|_| AppError::Unknown) // TODO: error handling
    }
}

pub enum ActorCommand<M> {
    Message(M),
}
