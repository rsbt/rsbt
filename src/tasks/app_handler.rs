use crate::{
    commands::{Command, QuitCommandRequest},
    methods::AnyRequest,
    tasks::{App, OneshotChannel, Sender},
    RsbtResult,
};
use std::any::Any;

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

    pub async fn request<C: 'static, R>(&mut self, command_request: C) -> RsbtResult<R>
    where
        C: AnyRequest<A>,
        R: 'static,
    {
        let (sender, receiver) = A::AnyResultOneshotChannel::create();

        self.send(Command::Request(sender, Box::new(command_request)))
            .await?;

        let result = receiver.await?;

        if let Ok(any) = <Box<dyn Any + Send>>::downcast::<R>(result) {
            Ok(*any)
        } else {
            Err(anyhow::anyhow!(
                "cannot downcast from request, caller and cally types do not match"
            ))
        }
    }

    pub async fn quit(&mut self) -> RsbtResult<()> {
        self.request(QuitCommandRequest).await
    }
}

#[cfg(test)]
mod tests {
    use crate::methods::AnyResult;
    use std::any::Any;

    #[test]
    fn check_downcast() {
        let q: AnyResult = Box::new(String::from("qqq"));

        let res = <Box<dyn Any + Send>>::downcast::<String>(q);
        let res = *res.unwrap();
        assert_eq!(String::from("qqq"), res);
    }
}
