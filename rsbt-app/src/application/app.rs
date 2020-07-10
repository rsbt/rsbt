use crate::{
    torrent::{TorrentProcess, TorrentProcessStatus},
    transport::IncomingConnection,
    RsbtResult, SHA1_SIZE,
};
use async_trait::async_trait;
use futures::{
    future::{abortable, join, BoxFuture},
    StreamExt,
};
use log::error;
use rsbt_bencode::Torrent;
use std::{convert::TryInto, path::PathBuf, sync::Arc};

/*
    async fn add_torrent(
        &mut self,
        data: Vec<u8>,
        filename: String,
        state: TorrentProcessStatus,
    ) -> RsbtResult<TorrentToken> {
        self.torrent_manager()
            .add_torrent(data, filename, state)
            .await
    }

    fn find_torrent_by_hash_id(&self, hash_id: &[u8; SHA1_SIZE]) -> Option<&TorrentProcess> {
        todo!()
    }
*/