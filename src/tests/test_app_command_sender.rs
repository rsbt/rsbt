use crate::{
    application::AppCommandSender,
    bridge::Sender,
    commands::Command,
    tests::{TestApp, TestSender},
    RsbtResult,
};
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct TestAppCommandSender;

impl AppCommandSender<TestApp> for TestAppCommandSender {}

#[async_trait]
impl Sender<Command<TestApp>, RsbtResult<()>> for TestAppCommandSender {
    async fn send(&mut self, m: Command<TestApp>) -> RsbtResult<()> {
        todo!()
    }
}
