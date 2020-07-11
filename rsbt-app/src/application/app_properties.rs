use std::net::SocketAddr;

pub trait AppProperties: Send + Sync {
    fn listen_addr(&self) -> &SocketAddr;
}
