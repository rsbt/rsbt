use crate::{
    application::AppCommandReceiver,
    bridge::Receiver,
    commands::Command,
    tests::{TestApp, TestReceiver},
};
use futures::stream::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Debug)]
pub struct TestAppCommandReceiver;

impl AppCommandReceiver<TestApp> for TestAppCommandReceiver {}

impl Receiver<Command<TestApp>> for TestAppCommandReceiver {}

impl Stream for TestAppCommandReceiver {
    type Item = Command<TestApp>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
