use rsbt_rt::Runtime;

use crate::{Actor, ActorHandle, App, AppError};

mod builder;
mod message_channel;

#[derive(Default)]
pub struct BlockingApp<R: Runtime> {
    app: App,
    runtime: R,
}

impl<R> BlockingApp<R>
where
    R: Runtime,
{
    pub fn builder() -> BlockingAppBuilder<R> {
        BlockingAppBuilder::default()
    }

    pub fn message_channel<T: Send + Unpin + 'static>(&self) -> BlockingMessageChannel<R, T> {
        BlockingMessageChannel {
            inner: self.app.message_channel(),
            handle: self.runtime.handle(),
        }
    }

    pub fn start<A: Actor<R>>(&self, actor: A) -> ActorHandle<A, R>
    where
        A::Message: Send + Unpin + 'static,
    {
        self.runtime.block_on(self.app.start(actor))
    }

    pub fn shutdown(self) -> Result<(), AppError> {
        self.runtime.shutdown_background();

        Ok(())
    }
}

pub use builder::BlockingAppBuilder;
pub use message_channel::BlockingMessageChannel;
