use crate::{torrent::TorrentToken, App, RsbtResult, TorrentProcessStatus};
use async_trait::async_trait;
use rsbt_bencode::Torrent;
use std::{convert::TryInto, path::PathBuf};

#[async_trait]
pub trait TorrentManager<A: App>: Send + private::TorrentManagerPriv {
    async fn add_torrent(
        &mut self,
        data: Vec<u8>,
        filename: String,
        state: TorrentProcessStatus,
    ) -> RsbtResult<TorrentToken> {
        let filepath = PathBuf::from(&filename);
        let name = filepath.file_stem().unwrap().to_string_lossy().into_owned();

        let torrent: Torrent = data.try_into()?;
        let hash_id = torrent.info_sha1_hash();
        let info = torrent.info()?;
        todo!()
    }
}

mod private {
    use crate::manager::DefaultTorrentManager;

    pub trait TorrentManagerPriv {}

    impl TorrentManagerPriv for DefaultTorrentManager {}
}
