use crate::{transport::SocketListener, RsbtResult};
use async_trait::async_trait;
use futures::{stream::BoxStream, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::compat::{Compat, Tokio02AsyncReadCompatExt};

#[async_trait]
impl SocketListener for BoxStream<'static, RsbtResult<Compat<TcpStream>>> {
    async fn bind(addr: SocketAddr) -> RsbtResult<Self> {
        Ok(TcpListener::bind(addr)
            .await?
            .map(|x| x.map(|x| x.compat()).map_err(anyhow::Error::from))
            .boxed())
    }
}
