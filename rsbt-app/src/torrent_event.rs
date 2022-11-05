use rsbt_rt::RuntimeHandle;

use crate::actor::DownloadEvent;

#[derive(Clone)]
pub enum TorrentEvent<R: RuntimeHandle> {
    Download(DownloadEvent<R>),
}

impl<R> From<DownloadEvent<R>> for TorrentEvent<R>
where
    R: RuntimeHandle,
{
    fn from(value: DownloadEvent<R>) -> Self {
        Self::Download(value)
    }
}
