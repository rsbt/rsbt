use crate::tasks::OneshotSender;
use std::fmt::Debug;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct TokioOneshotSender<M>(oneshot::Sender<M>);

impl<M: Send + Debug + 'static> OneshotSender<M> for TokioOneshotSender<M> {
    fn send(self, t: M) -> Result<(), M> {
        self.0.send(t)
    }
}

impl<M> From<oneshot::Sender<M>> for TokioOneshotSender<M> {
    fn from(value: oneshot::Sender<M>) -> Self {
        Self(value)
    }
}
