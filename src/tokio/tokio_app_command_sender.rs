use crate::{
    commands::Command,
    application::{AppCommandSender, Sender},
    tokio::{TokioApp, TokioSender},
    RsbtResult,
};
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct TokioAppCommandSender(TokioSender<Command<TokioApp>>);

impl AppCommandSender<TokioApp> for TokioAppCommandSender {}

#[async_trait]
impl Sender<Command<TokioApp>, RsbtResult<()>> for TokioAppCommandSender {
    async fn send(&mut self, m: Command<TokioApp>) -> RsbtResult<()> {
        self.0.send(m).await?;
        Ok(())
    }
}

impl From<TokioSender<Command<TokioApp>>> for TokioAppCommandSender {
    fn from(value: TokioSender<Command<TokioApp>>) -> Self {
        Self(value)
    }
}
