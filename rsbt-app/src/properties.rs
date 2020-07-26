use crate::application::AppProperties;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, Serialize, Deserialize)]
pub struct RsbtAppProperties {
    listen_addr: SocketAddr,
}

impl Default for RsbtAppProperties {
    fn default() -> Self {
        Self {
            // listen_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6881),
            listen_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080),
        }
    }
}

impl AppProperties for RsbtAppProperties {
    fn listen_addr(&self) -> &SocketAddr {
        &self.listen_addr
    }

    fn mpsc_buffer_size(&self) -> usize {
        10
    }
}
