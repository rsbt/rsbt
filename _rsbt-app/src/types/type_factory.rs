use crate::{
    types::{OneshotSender, Sender},
    RsbtResult,
};
use futures::Stream;
use std::future::Future;

pub trait TypeFactory<M> {
    type MpscSender: Sender<M, Error = anyhow::Error> + Clone + Unpin + Send + Sync;
    type MpscReceiver: Stream<Item = M> + Unpin + Send + Sync;
    type OneshotSender: OneshotSender<M> + Unpin + Send + Sync;
    type OneshotReceiver: Future<Output = RsbtResult<M>> + Unpin + Send + Sync;

    fn mpsc_channel(buffer: usize) -> (Self::MpscSender, Self::MpscReceiver);
    fn oneshot_channel() -> (Self::OneshotSender, Self::OneshotReceiver);
}
