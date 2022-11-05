use crate::actor::DownloadEvent;

#[derive(Clone)]
pub enum TorrentEvent {
    Download(DownloadEvent),
}

impl From<DownloadEvent> for TorrentEvent {
    fn from(value: DownloadEvent) -> Self {
        Self::Download(value)
    }
}
