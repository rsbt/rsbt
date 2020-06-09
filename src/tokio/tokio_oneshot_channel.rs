use crate::{
    bridge::OneshotChannel,
    tokio::{TokioApp, TokioOneshotReceiver, TokioOneshotSender},
    App,
};
use std::{fmt::Debug, marker::PhantomData};
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct TokioOneshotChannel<M, A = TokioApp>(PhantomData<(M, A)>);

impl<M: Send + Debug + 'static, A: App + Debug> OneshotChannel<A, M> for TokioOneshotChannel<M, A> {
    type OneshotSender = TokioOneshotSender<M>;
    type OneshotReceiver = TokioOneshotReceiver<M>;

    fn create() -> (TokioOneshotSender<M>, TokioOneshotReceiver<M>) {
        let (sender, receiver) = oneshot::channel();
        (sender.into(), receiver.into())
    }
}
