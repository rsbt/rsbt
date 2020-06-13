use crate::{
    bridge::SocketStream,
    torrent::{TorrentHeader, TorrentProcess},
    RsbtResult,
};
use std::sync::Arc;

pub struct TorrentToken {
    header: Arc<TorrentHeader>,
}

impl TorrentToken {
    pub(crate) fn handshake(&self) -> &[u8] {
        todo!()
    }

    pub(crate) async fn accept_peer_connection<S: SocketStream>(
        &mut self,
        socket: S,
    ) -> RsbtResult<()> {
        todo!()
    }
}

impl From<Arc<TorrentHeader>> for TorrentToken {
    fn from(header: Arc<TorrentHeader>) -> Self {
        Self { header }
    }
}
