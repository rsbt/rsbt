use std::{future::Future, pin::Pin};

use rsbt_rt::RuntimeHandle;

use crate::{Actor, ActorHandle, Input, Output, TorrentEvent};

pub struct Download<I: Input, O: Output> {
    input: I,
    output: O,
}

impl<I: Input, O: Output> Download<I, O> {
    pub fn new(input: I, output: O) -> Self {
        Self { input, output }
    }
}

impl<I: Input, O: Output, R: RuntimeHandle> Actor<R> for Download<I, O> {
    type Message = DownloadEvent;

    fn handle_message(&mut self, msg: Self::Message) {
        todo!()
    }

    fn message_loop(
        actor: Self,
        receiver: R::MpscReceiver<Self::Message>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        Box::pin(async {})
    }
}

pub struct DownloadHandle<R: RuntimeHandle> {
    sender: R::MpscSender<DownloadEvent>,
}

pub enum DownloadEvent {}
