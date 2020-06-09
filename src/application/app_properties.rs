use std::{net::SocketAddr, fmt::Debug};

pub trait AppProperties: Send + Sync + Debug {

    fn listen_addr(&self) -> &SocketAddr;

}
