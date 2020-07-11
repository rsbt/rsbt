use crate::types::OneshotSender;
use std::fmt::{Debug, Formatter};

pub struct TokioOneshotSender<M>(pub(crate) tokio::sync::oneshot::Sender<M>);

impl<M> Debug for TokioOneshotSender<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokioOneshotSender")
    }
}

impl<M> OneshotSender<M> for TokioOneshotSender<M> {
    fn send(self, message: M) -> Result<(), M> {
        self.0.send(message)
    }
}
