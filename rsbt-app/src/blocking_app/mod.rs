use crate::{tokio::Runtime, Actor, ActorHandle, App, AppError};

mod builder;
mod message_channel;

pub struct BlockingApp {
    app: App,
    runtime: Runtime,
}

impl BlockingApp {
    pub fn builder() -> BlockingAppBuilder {
        BlockingAppBuilder::default()
    }

    pub fn message_channel<T>(&self) -> BlockingMessageChannel<T> {
        BlockingMessageChannel {
            inner: self.app.message_channel(),
            handle: self.runtime.handle().clone(),
        }
    }

    pub fn start<A>(&self, actor: A) -> ActorHandle<A>
    where
        A: Actor + Send + 'static,
        A::Message: Send,
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
