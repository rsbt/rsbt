use super::{AnyCommand, AnyResult, Command};
use crate::{application::AppTypeFactory, types::TypeFactory, RsbtResult};
use async_trait::async_trait;
use futures::{future::BoxFuture, Sink, SinkExt};
use std::{any::Any, fmt::Debug};

#[async_trait]
pub trait CommandSender<A, B>:
    Sink<Command<A, B>, Error = anyhow::Error> + Clone + Debug + Unpin + Send + Sync
where
    A: AppTypeFactory,
    B: Sync + Send + 'static,
{
    async fn request<F, R>(&mut self, f: F) -> RsbtResult<R>
    where
        F: FnOnce(&mut B) -> BoxFuture<'_, R> + Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        let (sender, receiver) = <A as TypeFactory<AnyResult>>::oneshot_channel();

        self.send(Command::Request(
            sender,
            Box::new(AnyCommand(Some(Box::new(f)))),
        ))
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
}

impl<T, A: AppTypeFactory, B> CommandSender<A, B> for T
where
    B: Sync + Send + 'static,
    T: Sink<Command<A, B>, Error = anyhow::Error> + Clone + Debug + Unpin + Send + Sync,
{
}