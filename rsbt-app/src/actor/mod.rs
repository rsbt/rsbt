use std::marker::PhantomData;

use thiserror::Error;

use crate::{
    tokio::{mpsc_channel, spawn, MpscReceiver, MpscSender},
    AppError, DEFAULT_CHANNEL_BUFFER,
};

mod download;
pub use download::{Download, DownloadHandle, DownloadMessage};

#[trait_variant::make(Actor: Send)]
pub trait LocalActor {
    type Message;

    async fn handle_message(&mut self, msg: Self::Message);
}

pub async fn start<A>(actor: A) -> ActorHandle<A>
where
    A: Actor + Send + 'static,
    A::Message: Send,
{
    let (sender, receiver) = mpsc_channel(DEFAULT_CHANNEL_BUFFER);
    let _ = spawn(ActorMessageLoop::message_loop(actor, receiver));
    ActorHandle::new(sender)
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
                ActorCommand::Message(message) => actor.handle_message(message).await,
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

    pub async fn send(&self, message: A::Message) -> Result<(), ActorError> {
        self.sender
            .send(ActorCommand::Message(message))
            .await
            .map_err(|_| ActorError::Send)
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

#[derive(Debug, Error)]
pub enum ActorError {
    #[error("Could not send message to actor")]
    Send,
}
