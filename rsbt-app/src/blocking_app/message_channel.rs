use crate::{
    actor::{EventSubscription, Publisher},
    app::MessageChannel,
    tokio::{Handle, MpscReceiver},
    ActorHandle, AppError,
};

pub struct BlockingMessageChannel<T> {
    pub(super) inner: MessageChannel<T>,
    pub(super) handle: Handle,
}

impl<T> BlockingMessageChannel<T> {
    pub fn subscribe<A>(&self, actor_handle: ActorHandle<A>) -> Result<(), AppError>
    where
        A: Publisher<Event = T>,
        A::Message: EventSubscription<Event = T>,
    {
        self.handle.block_on(self.inner.subscribe(actor_handle))
    }

    pub fn listen(self) -> BlockingMessageReceiver<T> {
        let Self { handle, inner } = self;

        let inner = inner.listen();

        BlockingMessageReceiver { inner, handle }
    }
}

pub struct BlockingMessageReceiver<T> {
    pub(super) inner: MpscReceiver<T>,
    pub(super) handle: Handle,
}

impl<T: Send> BlockingMessageReceiver<T> {
    pub fn next(&mut self) -> Option<T> {
        self.handle.block_on(self.inner.recv())
    }
}
