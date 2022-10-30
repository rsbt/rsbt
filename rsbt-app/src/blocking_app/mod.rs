use rsbt_rt::Runtime;

use crate::{Actor, App, AppError};

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

    pub fn start<A: Actor>(&self, actor: A) -> Result<A::Handle, AppError> {
        // _ = self.runtime.spawn(future)
        todo!();
    }

    pub fn shutdown(&self) -> Result<(), AppError> {
        todo!();
    }
}

pub use builder::BlockingAppBuilder;
pub use message_channel::BlockingMessageChannel;
