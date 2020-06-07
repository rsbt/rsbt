use crate::{commands::Command, tasks::Sender, RsbtResult};
use async_trait::async_trait;

#[async_trait]
pub trait AppCommandSender: Sender<Command, RsbtResult<()>> {}
