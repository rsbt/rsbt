use crate::bridge::SocketStream;
use futures::{AsyncRead, AsyncWrite};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, Tokio02AsyncReadCompatExt};

#[derive(Debug)]
pub struct TokioSocketStream(Compat<TcpStream>);

impl SocketStream for TokioSocketStream {}

impl AsyncWrite for TokioSocketStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_close(cx)
    }
}

impl AsyncRead for TokioSocketStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

impl From<TcpStream> for TokioSocketStream {
    fn from(value: TcpStream) -> Self {
        Self(value.compat())
    }
}
