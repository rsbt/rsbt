use crate::{
    commands::Command,
    tasks::{AppCommandSender, Sender},
    tokio::TokioSender,
    RsbtResult,
};
use async_trait::async_trait;

#[derive(Clone)]
pub struct TokioAppCommandSender(TokioSender<Command>);

impl AppCommandSender for TokioAppCommandSender {}

#[async_trait]
impl Sender<Command, RsbtResult<()>> for TokioAppCommandSender {
    async fn send(&mut self, m: Command) -> RsbtResult<()> {
        self.0.send(m).await?;
        Ok(())
    }
}

impl From<TokioSender<Command>> for TokioAppCommandSender {
    fn from(value: TokioSender<Command>) -> Self {
        Self(value)
    }
}
