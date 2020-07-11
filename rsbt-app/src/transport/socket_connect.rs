use crate::RsbtResult;
use async_trait::async_trait;
use std::net::SocketAddr;

#[async_trait]
pub trait SocketConnect<SS> {
    async fn connect(addr: SocketAddr) -> RsbtResult<SS>;
}
