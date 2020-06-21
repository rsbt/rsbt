use crate::App;

pub trait TorrentManager<A: App>: private::TorrentManagerPriv {}

mod private {
    use crate::manager::DefaultTorrentManager;

    pub trait TorrentManagerPriv {}

    impl TorrentManagerPriv for DefaultTorrentManager {}
}
