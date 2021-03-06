use crate::RsbtResult;
use futures::Future;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct TokioOneshotReceiver<M>(pub(crate) tokio::sync::oneshot::Receiver<M>);

impl<M> Future for TokioOneshotReceiver<M> {
    type Output = RsbtResult<M>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx).map_err(anyhow::Error::from)
    }
}
