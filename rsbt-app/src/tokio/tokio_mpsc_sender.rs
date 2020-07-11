use futures::Sink;
use std::{
    fmt::{Debug, Formatter},
    pin::Pin,
    task::{Context, Poll},
};

pub struct TokioMpscSender<M>(pub(crate) tokio::sync::mpsc::Sender<M>);

impl<M> Clone for TokioMpscSender<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<M> Debug for TokioMpscSender<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokioMpscSender({:?})", self.0)
    }
}

impl<M> Sink<M> for TokioMpscSender<M>
where
    M: Send + Sync,
{
    type Error = anyhow::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx).map_err(anyhow::Error::from)
    }

    fn start_send(mut self: Pin<&mut Self>, item: M) -> Result<(), Self::Error> {
        self.0.try_send(item).map_err(|e| anyhow::anyhow!("{}", e))
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self.0.poll_ready(cx) {
            Poll::Ready(Err(_)) => Poll::Ready(Ok(())),
            x => x.map_err(anyhow::Error::from),
        }
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}
