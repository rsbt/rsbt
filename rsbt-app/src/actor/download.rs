use std::{future::Future, pin::Pin};

use rsbt_rt::RuntimeHandle;

use crate::{Actor, ActorHandle, Input, Output, TorrentEvent};

use super::{EventSubscription, Publisher};

pub struct Download<I: Input, O: Output, R: RuntimeHandle> {
    input: I,
    output: O,
    torrent_event_senders: Vec<R::MpscSender<TorrentEvent<R>>>,
}

impl<I: Input, O: Output, R: RuntimeHandle> Download<I, O, R> {
    pub fn new(input: I, output: O) -> Self {
        Self {
            input,
            output,
            torrent_event_senders: vec![],
        }
    }

    async fn notify(&mut self, event: TorrentEvent<R>) {
        for torrent_event_sender in &mut self.torrent_event_senders {
            use futures::SinkExt;
            let _ = torrent_event_sender.send(event.clone()).await;
        }
    }
}

impl<I: Input + Send + 'static, O: Output + Send + 'static, R: RuntimeHandle> Actor<R>
    for Download<I, O, R>
{
    type Message = DownloadEvent<R>;

    fn handle_message(&mut self, msg: Self::Message) {
        todo!()
    }

    fn message_loop(
        mut actor: Self,
        mut receiver: R::MpscReceiver<Self::Message>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        Box::pin(async move {
            use futures::StreamExt;
            while let Some(message) = receiver.next().await {
                actor.handle_message(message);
            }
        })
    }
}

impl<I: Input + Send + 'static, O: Output + Send + 'static, R: RuntimeHandle> Publisher<R>
    for Download<I, O, R>
{
    type Event = TorrentEvent<R>;

    fn register_subscriber(&mut self, sender: <R as RuntimeHandle>::MpscSender<Self::Event>) {
        self.torrent_event_senders.push(sender)
    }
}

pub struct DownloadHandle<R: RuntimeHandle> {
    sender: R::MpscSender<DownloadEvent<R>>,
}

#[derive(Clone)]
pub enum DownloadEvent<R: RuntimeHandle> {
    Started,
    Subscribe(R::MpscSender<TorrentEvent<R>>),
}

impl<R> EventSubscription<R> for DownloadEvent<R>
where
    R: RuntimeHandle,
{
    type Event = TorrentEvent<R>;

    fn message(sender: R::MpscSender<Self::Event>) -> Self
    where
        Self: Sized + Send,
    {
        todo!()
    }
}
