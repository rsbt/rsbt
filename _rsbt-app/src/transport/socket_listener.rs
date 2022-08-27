use crate::RsbtResult;
use async_trait::async_trait;
use std::net::SocketAddr;

#[async_trait]
pub trait SocketListener: Sized {
    async fn bind(addr: SocketAddr) -> RsbtResult<Self>;
}
