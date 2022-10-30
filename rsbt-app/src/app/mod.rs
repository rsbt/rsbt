use rsbt_rt::Runtime;

mod message_channel;

pub use self::message_channel::MessageChannel;

#[derive(Default)]
pub struct App {}

impl App {
    pub fn message_channel<R: Runtime, T: Send + Unpin + 'static>(&self) -> MessageChannel<R, T> {
        MessageChannel::new(100)
    }
}
