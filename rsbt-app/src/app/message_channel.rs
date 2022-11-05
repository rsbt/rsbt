use futures::Sink;
use rsbt_rt::{Runtime, RuntimeError};

use crate::{
    actor::{EventSubscription, Publisher},
    ActorHandle, AppError,
};

pub struct MessageChannel<R: Runtime, T: Send + Unpin + 'static> {
    mpsc_receiver: R::MpscReceiver<T>,
    mpsc_sender: R::MpscSender<T>,
}

impl<R: Runtime, T> MessageChannel<R, T>
where
    T: Send + Unpin + 'static,
{
    pub fn new(buffer: usize) -> Self {
        let (mpsc_sender, mpsc_receiver) = R::channel(buffer);

        Self {
            mpsc_receiver,
            mpsc_sender,
        }
    }

    pub fn listen(self) -> R::MpscReceiver<T> {
        self.mpsc_receiver
    }

    pub async fn subscribe<A>(&self, mut actor_handle: ActorHandle<A, R>) -> Result<(), AppError>
    where
        A: Publisher<R, Event = T>,
        A::Message: EventSubscription<R, Event = T>,
        RuntimeError: From<<R::MpscSender<A::Message> as Sink<A::Message>>::Error>,
    {
        actor_handle
            .subscribe(self.mpsc_sender.clone())
            .await
            .map_err(RuntimeError::from)
            .map_err(AppError::Runtime)
    }
}
