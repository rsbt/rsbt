use crate::{
    commands::{Command, CommandRequest},
    tasks::{App, Sender},
    RsbtResult,
};

#[derive(Clone, Debug)]
pub struct AppHandler<A: App>(A::CommandSender);

impl<A> AppHandler<A>
where
    A: App,
{
    pub fn new(from: A::CommandSender) -> Self {
        Self(from)
    }

    pub async fn send(&mut self, command: Command<A>) -> RsbtResult<()> {
        self.0.send(command).await
    }

    //    pub async fn request<C, R>(&mut self, command_request: C) -> R where C: CommandRequest {
    //    }
}
