use crate::types::OneshotSender;

pub struct TokioOneshotSender<M>(pub(crate) tokio::sync::oneshot::Sender<M>);

impl<M> OneshotSender<M> for TokioOneshotSender<M> {
    fn send(self, message: M) -> Result<(), M> {
        self.0.send(message)
    }
}
