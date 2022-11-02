use crate::actor::DownloadEvent;

pub enum TorrentEvent {
    Download(DownloadEvent),
}

impl From<DownloadEvent> for TorrentEvent {
    fn from(value: DownloadEvent) -> Self {
        Self::Download(value)
    }
}
