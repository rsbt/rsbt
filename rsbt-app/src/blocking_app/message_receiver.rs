use crate::{ActorHandle, AppError, Message};

pub struct BlockingMessageReceiver {}

impl BlockingMessageReceiver {
    pub fn subscribe<H: ActorHandle>(&self, handle: H) -> Result<(), AppError> {
        todo!();
    }
}

impl Iterator for BlockingMessageReceiver {
    type Item = Result<Message, AppError>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
