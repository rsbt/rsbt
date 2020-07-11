use super::OneshotSender;
use crate::RsbtResult;
use futures::{Sink, Stream};
use std::{fmt::Debug, future::Future};

pub trait TypeFactory<M> {
    type MpscSender: Sink<M, Error = anyhow::Error> + Clone + Debug + Unpin + Send + Sync;
    type MpscReceiver: Stream<Item = M> + Debug + Unpin + Send + Sync;
    type OneshotSender: OneshotSender<M> + Debug + Unpin + Send + Sync;
    type OneshotReceiver: Future<Output = RsbtResult<M>> + Debug + Unpin + Send + Sync;

    fn mpsc_channel(buffer: usize) -> (Self::MpscSender, Self::MpscReceiver);
    fn oneshot_channel() -> (Self::OneshotSender, Self::OneshotReceiver);
}
