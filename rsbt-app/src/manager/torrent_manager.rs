use crate::App;

pub trait TorrentManager<A: App>: private::TorrentManagerPriv {}

mod private {
    pub trait TorrentManagerPriv {}
}
