use std::{future::Future, pin::Pin};

use crate::{
    tokio::{MpscReceiver, MpscSender},
    Actor, Input, Output, TorrentEvent,
};

use super::{EventSubscription, Publisher};

pub struct Download<I: Input, O: Output> {
    input: I,
    output: O,
    torrent_event_senders: Vec<MpscSender<TorrentEvent>>,
}

impl<I: Input, O: Output> Download<I, O> {
    pub fn new(input: I, output: O) -> Self {
        Self {
            input,
            output,
            torrent_event_senders: vec![],
        }
    }

    async fn notify(&mut self, event: TorrentEvent) {
        for torrent_event_sender in &mut self.torrent_event_senders {
            use futures::SinkExt;
            let _ = torrent_event_sender.send(event.clone()).await;
        }
    }
}

impl<I: Input + Send + 'static, O: Output + Send + 'static> Actor for Download<I, O> {
    type Message = DownloadEvent;

    fn handle_message(&mut self, msg: Self::Message) {
        match msg {
            DownloadEvent::Subscribe(sender) => self.register_subscriber(sender),
            _ => (),
        }
    }

    fn message_loop(
        mut actor: Self,
        mut receiver: MpscReceiver<Self::Message>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        Box::pin(async move {
            while let Some(message) = receiver.recv().await {
                actor.handle_message(message);
            }
        })
    }
}

impl<I: Input + Send + 'static, O: Output + Send + 'static> Publisher for Download<I, O> {
    type Event = TorrentEvent;

    fn register_subscriber(&mut self, sender: MpscSender<Self::Event>) {
        self.torrent_event_senders.push(sender)
    }
}

pub struct DownloadHandle {
    sender: MpscSender<DownloadEvent>,
}

#[derive(Clone)]
pub enum DownloadEvent {
    Started,
    Subscribe(MpscSender<TorrentEvent>),
}

impl EventSubscription for DownloadEvent {
    type Event = TorrentEvent;

    fn message(sender: MpscSender<Self::Event>) -> Self
    where
        Self: Sized + Send,
    {
        Self::Subscribe(sender)
    }
}
