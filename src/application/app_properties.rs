use std::{fmt::Debug, net::SocketAddr};

pub trait AppProperties: Send + Sync + Debug {
    fn listen_addr(&self) -> &SocketAddr;
}
