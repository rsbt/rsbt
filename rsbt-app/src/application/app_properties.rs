use std::{net::SocketAddr, path::Path};

pub trait AppProperties: Send + Sync {
    fn listen_addr(&self) -> &SocketAddr;
    fn mpsc_buffer_size(&self) -> usize;
}
