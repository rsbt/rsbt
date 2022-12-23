use async_trait::async_trait;

use crate::{tokio::MpscSender, Actor, Input, Output, TorrentEvent};

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
            let _ = torrent_event_sender.send(event.clone()).await;
        }
    }
}

#[async_trait]
impl<I: Input + Send + 'static, O: Output + Send + 'static> Actor for Download<I, O> {
    type Message = DownloadMessage;

    async fn handle_message(&mut self, msg: Self::Message) {
        match msg {
            DownloadMessage::Subscribe(sender) => self.register_subscriber(sender),
            _ => (),
        }
    }
}

impl<I: Input + Send + 'static, O: Output + Send + 'static> Publisher for Download<I, O> {
    type Event = TorrentEvent;

    fn register_subscriber(&mut self, sender: MpscSender<Self::Event>) {
        self.torrent_event_senders.push(sender)
    }
}

pub struct DownloadHandle {
    sender: MpscSender<DownloadMessage>,
}

#[derive(Clone)]
pub enum DownloadMessage {
    Subscribe(MpscSender<TorrentEvent>),
}

impl EventSubscription for DownloadMessage {
    type Event = TorrentEvent;

    fn message(sender: MpscSender<Self::Event>) -> Self
    where
        Self: Sized + Send,
    {
        Self::Subscribe(sender)
    }
}
