use rsbt_rt::Runtime;

use crate::{Actor, App, AppError};

mod builder;
mod message_receiver;

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
        BlockingAppBuilder { runtime: None }
    }

    pub fn message_receiver(&self) -> BlockingMessageReceiver {
        BlockingMessageReceiver {}
    }

    pub fn start<A: Actor>(&self, actor: A) -> Result<A::Handle, AppError> {
        todo!();
    }

    pub fn shutdown(&self) -> Result<(), AppError> {
        todo!();
    }
}

pub use builder::BlockingAppBuilder;
pub use message_receiver::BlockingMessageReceiver;
