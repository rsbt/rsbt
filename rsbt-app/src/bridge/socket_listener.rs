use crate::{
    bridge::{Receiver, SocketStream},
    RsbtResult,
};
use async_trait::async_trait;
use std::net::SocketAddr;

#[async_trait]
pub trait SocketListener<SS: SocketStream>: Receiver<RsbtResult<SS>> + Sized {
    async fn bind(addr: SocketAddr) -> RsbtResult<Self>;
}
