use crate::{
    commands::Command,
    tasks::{AppCommandReceiver, Receiver},
    tokio::TokioReceiver,
};
use futures::stream::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct TokioAppCommandReceiver(TokioReceiver<Command>);

impl AppCommandReceiver for TokioAppCommandReceiver {}

impl Receiver<Command> for TokioAppCommandReceiver {}

impl Stream for TokioAppCommandReceiver {
    type Item = Command;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

impl From<TokioReceiver<Command>> for TokioAppCommandReceiver {
    fn from(value: TokioReceiver<Command>) -> Self {
        Self(value)
    }
}
