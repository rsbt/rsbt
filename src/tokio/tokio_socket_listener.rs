use crate::{
    bridge::{Receiver, SocketListener},
    tokio::TokioSocketStream,
    RsbtResult,
};
use async_trait::async_trait;
use futures::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::TcpListener;

pub struct TokioSocketListener(TcpListener);

#[async_trait]
impl SocketListener<TokioSocketStream> for TokioSocketListener {
    async fn bind(addr: std::net::SocketAddr) -> RsbtResult<Self> {
        TcpListener::bind(addr)
            .await
            .map(TokioSocketListener::from)
            .map_err(anyhow::Error::from)
    }
}

impl Receiver<RsbtResult<TokioSocketStream>> for TokioSocketListener {}

impl Stream for TokioSocketListener {
    type Item = RsbtResult<TokioSocketStream>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_next(cx) {
            Poll::Ready(Some(Ok(t))) => Poll::Ready(Some(Ok(TokioSocketStream::from(t)))),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(anyhow::Error::from(e)))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl From<TcpListener> for TokioSocketListener {
    fn from(value: TcpListener) -> Self {
        Self(value)
    }
}
