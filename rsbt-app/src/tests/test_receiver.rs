use crate::bridge::Receiver;
use futures::Stream;
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Debug)]
pub struct TestReceiver<M>(PhantomData<M>);

impl<M: Send + Unpin> Receiver<M> for TestReceiver<M> {}

impl<M> Stream for TestReceiver<M> {
    type Item = M;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
