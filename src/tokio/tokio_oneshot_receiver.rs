use crate::{application::OneshotReceiver, RsbtResult};
use futures::Future;
use std::{
    fmt::Debug,
    pin::Pin,
    task::{self, Poll},
};
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct TokioOneshotReceiver<M>(oneshot::Receiver<M>);

impl<M: Debug> OneshotReceiver<M> for TokioOneshotReceiver<M> {}

impl<M> Future for TokioOneshotReceiver<M> {
    type Output = RsbtResult<M>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx).map_err(anyhow::Error::from)
    }
}

impl<M> From<oneshot::Receiver<M>> for TokioOneshotReceiver<M> {
    fn from(value: oneshot::Receiver<M>) -> Self {
        Self(value)
    }
}
