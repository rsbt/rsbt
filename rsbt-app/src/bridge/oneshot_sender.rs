use std::fmt::Debug;

pub trait OneshotSender<M>: Send + Debug + 'static {
    fn send(self, t: M) -> Result<(), M>;
}
