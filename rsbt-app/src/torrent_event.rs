use crate::actor::DownloadMessage;

#[derive(Clone)]
pub enum TorrentEvent {
    Download(DownloadMessage),
}

impl From<DownloadMessage> for TorrentEvent {
    fn from(value: DownloadMessage) -> Self {
        Self::Download(value)
    }
}
