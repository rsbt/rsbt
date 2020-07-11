use super::{App, AppProperties, AppRuntime};
use crate::{
    commands::{AnyResult, Command},
    transport::{SocketConnect, SocketListener},
    types::TypeFactory,
    RsbtResult,
};
use futures::{AsyncRead, AsyncWrite, Stream};

pub trait AppTypeFactory:
    Sized
    + TypeFactory<String>
    + TypeFactory<AnyResult>
    + TypeFactory<Command<Self, App<Self>>>
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
