use crate::{
    application::App,
    bridge::{OneshotChannel, Sender},
    commands::{Command, CommandRequestAny, CommandRequestFindTorrentByHashId, CommandRequestQuit},
    methods::AnyRequest,
    torrent::TorrentToken,
    RsbtResult, SHA1_SIZE,
};
use futures::{
    future::{join, BoxFuture, FutureExt},
    Future, StreamExt,
};
use log::debug;
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

    pub async fn request<'a, C: 'static, R>(&'a mut self, command_request: C) -> RsbtResult<R>
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
        self.request(CommandRequestQuit).await
    }

    pub async fn quit2(&mut self, params: Vec<String>) -> RsbtResult<()> {
        self.request(CommandRequestAny(Some(Box::new(|x: &mut A| {
            async move {
                debug!("some really shitty shit: {}", params[0]);
                x.quit().await
            }
            .boxed()
        }))))
        .await
    }

    pub async fn find_torrent_by_hash_id(
        &mut self,
        hash_id: &[u8; SHA1_SIZE],
    ) -> RsbtResult<Option<TorrentToken>> {
        self.request(CommandRequestFindTorrentByHashId::from(hash_id))
            .await
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
