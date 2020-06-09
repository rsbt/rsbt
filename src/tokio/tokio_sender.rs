use crate::{application::Sender, RsbtResult};
use async_trait::async_trait;
use std::fmt::Debug;
use tokio::sync::mpsc;

#[derive(Debug)]
pub(crate) struct TokioSender<M>(mpsc::Sender<M>);

impl<M> Clone for TokioSender<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[async_trait]
impl<M: Debug + Sync + Send + 'static> Sender<M, RsbtResult<()>> for TokioSender<M> {
    async fn send(&mut self, m: M) -> RsbtResult<()> {
        self.0.send(m).await?;
        Ok(())
    }
}

impl<M> From<mpsc::Sender<M>> for TokioSender<M> {
    fn from(value: mpsc::Sender<M>) -> Self {
        Self(value)
    }
}
