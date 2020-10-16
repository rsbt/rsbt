use crate::types::Sender;
use futures::{future::BoxFuture, FutureExt, TryFutureExt};
use std::fmt::Debug;

pub struct TokioMpscSender<M>(pub(crate) tokio::sync::mpsc::Sender<M>);

impl<M> Clone for TokioMpscSender<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<M> Sender<M> for TokioMpscSender<M>
where
    M: Send + Sync + Debug + 'static,
{
    type Error = anyhow::Error;

    fn send(&self, value: M) -> BoxFuture<Result<(), Self::Error>> {
        // TODO: migrate to tokio 0.3
        let mut sender = self.0.clone();
        async move { sender.send(value).map_err(anyhow::Error::from).await }.boxed()
    }
}
