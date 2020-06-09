use crate::{
    bridge::{Receiver, SocketListener},
    tests::TestSocketStream,
    RsbtResult,
};
use async_trait::async_trait;
use futures::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct TestSocketListener;

#[async_trait]
impl SocketListener<TestSocketStream> for TestSocketListener {
    async fn bind(addr: std::net::SocketAddr) -> RsbtResult<Self> {
        Ok(TestSocketListener)
    }
}

impl Receiver<RsbtResult<TestSocketStream>> for TestSocketListener {}

impl Stream for TestSocketListener {
    type Item = RsbtResult<TestSocketStream>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
