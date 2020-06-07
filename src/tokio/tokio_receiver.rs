use futures::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::mpsc;

pub struct TokioReceiver<M>(mpsc::Receiver<M>);

impl<M> Stream for TokioReceiver<M> {
    type Item = M;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.poll_recv(cx)
    }
}

impl<M> From<mpsc::Receiver<M>> for TokioReceiver<M> {
    fn from(value: mpsc::Receiver<M>) -> Self {
        Self(value)
    }
}
