use super::{App, AppProperties, AppRuntime};
use crate::{
    commands::{AnyResult, Command},
    managers::IncomingConnectionsManager,
    tokio::{
        TokioAppRuntime, TokioMpscSender, TokioOneshotReceiver, TokioOneshotSender,
        TokioSocketConnect,
    },
    transport::{SocketConnect, SocketListener},
    types::TypeFactory,
    RsbtAppProperties, RsbtResult,
};
use futures::{stream::BoxStream, AsyncRead, AsyncWrite, Stream};
use std::fmt::Debug;
use tokio::net::TcpStream;
use tokio_util::compat::Compat;

pub trait AppTypeFactory:
    Sized
    + TypeFactory<String>
    + TypeFactory<AnyResult>
    + TypeFactory<Command<Self, App<Self>>>
    + TypeFactory<Command<Self, IncomingConnectionsManager<Self>>>
    + Sync
    + Send
    + 'static
{
    type AppRuntime: AppRuntime;
    type SocketStream: Unpin + Send + AsyncRead + AsyncWrite;
    type SocketListener: SocketListener + Stream<Item = RsbtResult<Self::SocketStream>> + Unpin;
    type SocketConnect: SocketConnect<Self::SocketStream> + Unpin;
    type AppProperties: AppProperties;
}
