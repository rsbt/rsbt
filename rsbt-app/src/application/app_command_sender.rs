use crate::{application::App, bridge::Sender, commands::Command, RsbtResult};
use async_trait::async_trait;

#[async_trait]
pub trait AppCommandSender<A: App>: Sender<Command<A>, RsbtResult<()>> {}
