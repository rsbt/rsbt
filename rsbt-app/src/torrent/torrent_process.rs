use crate::torrent::TorrentToken;

pub struct TorrentProcess;

impl TorrentProcess {
    pub(crate) fn token(&self) -> TorrentToken {
        TorrentToken
    }
}
