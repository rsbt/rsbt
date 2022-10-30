use rsbt_rt::Runtime;

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
}
