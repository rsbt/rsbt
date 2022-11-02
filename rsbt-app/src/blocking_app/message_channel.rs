use rsbt_rt::{Runtime, RuntimeHandle};

use crate::{app::MessageChannel, Actor, ActorHandle, AppError};

pub struct BlockingMessageChannel<R: Runtime, T: Send + Unpin + 'static> {
    pub(super) inner: MessageChannel<R, T>,
    pub(super) handle: R::Handle,
}

impl<R, T> BlockingMessageChannel<R, T>
where
    R: Runtime,
    T: Send + Unpin + 'static,
{
    pub fn subscribe<A>(&self, actor_handle: ActorHandle<A, R>) -> Result<(), AppError>
    where
        A: Actor<R>,
    {
        todo!();
    }

    pub fn listen(self) -> BlockingMessageReceiver<R, T> {
        let Self { handle, inner } = self;

        let inner = inner.listen();

        BlockingMessageReceiver { inner, handle }
    }
}

pub struct BlockingMessageReceiver<R: Runtime, T: Send + Unpin + 'static> {
    pub(super) inner: R::MpscReceiver<T>,
    pub(super) handle: R::Handle,
}

impl<R: Runtime, T: Send + Unpin + 'static> BlockingMessageReceiver<R, T> {
    pub fn next(&mut self) -> Option<T> {
        use futures::StreamExt;
        self.handle.block_on(self.inner.next())
    }
}
