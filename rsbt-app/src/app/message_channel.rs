use crate::{
    actor::{EventSubscription, Publisher},
    tokio::{mpsc_channel, MpscReceiver, MpscSender},
    ActorHandle, AppError,
};

pub struct MessageChannel<T> {
    mpsc_receiver: MpscReceiver<T>,
    mpsc_sender: MpscSender<T>,
}

impl<T> MessageChannel<T> {
    pub fn new(buffer: usize) -> Self {
        let (mpsc_sender, mpsc_receiver) = mpsc_channel(buffer);

        Self {
            mpsc_receiver,
            mpsc_sender,
        }
    }

    pub fn listen(self) -> MpscReceiver<T> {
        self.mpsc_receiver
    }

    pub async fn subscribe<A>(&self, actor_handle: &mut ActorHandle<A>) -> Result<(), AppError>
    where
        A: Publisher<Event = T>,
        A::Message: EventSubscription<Event = T>,
    {
        actor_handle
            .subscribe(self.mpsc_sender.clone())
            .await
            .map_err(From::from)
    }
}
