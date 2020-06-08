use crate::{
    commands::{Command, QuitCommandRequest},
    methods::AnyRequest,
    tasks::{App, Sender},
    RsbtResult,
};

#[derive(Debug)]
pub struct AppHandler<A: App>(A::CommandSender);

impl<A: App> Clone for AppHandler<A> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

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

    pub async fn request<C, R>(&mut self, command_request: C) -> RsbtResult<R>
    where
        C: AnyRequest<A>,
    {
        todo!();
    }

    pub async fn quit(&mut self) -> RsbtResult<()> {
        self.request(QuitCommandRequest).await
    }
}
