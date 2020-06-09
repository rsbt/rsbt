use crate::bridge::SocketStream;
use futures::{AsyncRead, AsyncWrite};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct TestSocketStream;

impl SocketStream for TestSocketStream {}

impl AsyncWrite for TestSocketStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        todo!()
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        todo!()
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        todo!()
    }
}

impl AsyncRead for TestSocketStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        todo!()
    }
}
