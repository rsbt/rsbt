use crate::{
    bridge::OneshotChannel,
    tokio::{TokioApp, TokioOneshotReceiver, TokioOneshotSender},
};
use std::{fmt::Debug, marker::PhantomData};
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct TokioOneshotChannel<M>(PhantomData<M>);

impl<M: Send + Debug + 'static> OneshotChannel<TokioApp, M> for TokioOneshotChannel<M> {
    type OneshotSender = TokioOneshotSender<M>;
    type OneshotReceiver = TokioOneshotReceiver<M>;

    fn create() -> (TokioOneshotSender<M>, TokioOneshotReceiver<M>) {
        let (sender, receiver) = oneshot::channel();
        (sender.into(), receiver.into())
    }
}
