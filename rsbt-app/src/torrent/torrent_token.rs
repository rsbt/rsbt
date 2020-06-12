use crate::{bridge::SocketStream, RsbtResult};

pub struct TorrentToken;

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
