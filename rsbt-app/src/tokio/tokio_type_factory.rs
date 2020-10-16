use std::fmt::Debug;

use super::{
    TokioAppRuntime, TokioMpscSender, TokioOneshotReceiver, TokioOneshotSender, TokioSocketConnect,
};
use crate::{application::AppTypeFactory, types::TypeFactory, RsbtAppProperties, RsbtResult};
use futures::stream::BoxStream;
use tokio::net::TcpStream;
use tokio_util::compat::Compat;

pub struct TokioTypeFactory;

impl AppTypeFactory for TokioTypeFactory {
    type AppRuntime = TokioAppRuntime;
    type SocketStream = Compat<TcpStream>;
    type SocketListener = BoxStream<'static, RsbtResult<Self::SocketStream>>;
    type SocketConnect = TokioSocketConnect;
    type AppProperties = RsbtAppProperties;
}

impl<M> TypeFactory<M> for TokioTypeFactory
where
    M: Send + Sync + Debug + 'static,
{
    type MpscSender = TokioMpscSender<M>;
    type MpscReceiver = tokio::sync::mpsc::Receiver<M>;
    type OneshotSender = TokioOneshotSender<M>;
    type OneshotReceiver = TokioOneshotReceiver<M>;

    fn mpsc_channel(buffer: usize) -> (Self::MpscSender, Self::MpscReceiver) {
        let (sender, receiver) = tokio::sync::mpsc::channel(buffer);

        (TokioMpscSender(sender), receiver)
    }

    fn oneshot_channel() -> (Self::OneshotSender, Self::OneshotReceiver) {
        let (sender, receiver) = tokio::sync::oneshot::channel();

        (TokioOneshotSender(sender), TokioOneshotReceiver(receiver))
    }
}
