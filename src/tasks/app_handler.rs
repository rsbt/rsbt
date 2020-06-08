use crate::{
    commands::Command,
    tasks::{App, Sender},
    RsbtResult,
};

#[derive(Clone)]
pub struct AppHandler<A: App>(A::CommandSender);

impl<A> AppHandler<A>
where
    A: App,
{
    pub fn new(from: A::CommandSender) -> Self {
        Self(from)
    }

    pub async fn send(&mut self, command: Command) -> RsbtResult<()> {
        self.0.send(command).await
    }
}
