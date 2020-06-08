use crate::{
    commands::Command,
    tasks::{AppCommandReceiver, Receiver},
    tokio::{TokioApp, TokioReceiver},
};
use futures::stream::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Debug)]
pub struct TokioAppCommandReceiver(TokioReceiver<Command<TokioApp>>);

impl AppCommandReceiver<TokioApp> for TokioAppCommandReceiver {}

impl Receiver<Command<TokioApp>> for TokioAppCommandReceiver {}

impl Stream for TokioAppCommandReceiver {
    type Item = Command<TokioApp>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

impl From<TokioReceiver<Command<TokioApp>>> for TokioAppCommandReceiver {
    fn from(value: TokioReceiver<Command<TokioApp>>) -> Self {
        Self(value)
    }
}
