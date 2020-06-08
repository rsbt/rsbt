use crate::{
    commands::Command,
    tasks::{App, Sender},
    RsbtResult,
};
use async_trait::async_trait;

#[async_trait]
pub trait AppCommandSender<A: App>: Sender<Command<A>, RsbtResult<()>> {}
