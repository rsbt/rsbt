use crate::{transport::SocketConnect, RsbtResult};
use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, Tokio02AsyncReadCompatExt};

pub struct TokioSocketConnect;

#[async_trait]
impl SocketConnect<Compat<TcpStream>> for TokioSocketConnect {
    async fn connect(addr: SocketAddr) -> RsbtResult<Compat<TcpStream>> {
        TcpStream::connect(addr)
            .await
            .map(|x| x.compat())
            .map_err(anyhow::Error::from)
    }
}
