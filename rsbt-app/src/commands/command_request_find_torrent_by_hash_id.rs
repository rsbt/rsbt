use crate::{application::App, methods::Request, torrent::TorrentToken, SHA1_SIZE};
use async_trait::async_trait;

#[derive(Debug)]
pub struct CommandRequestFindTorrentByHashId([u8; SHA1_SIZE]);

#[async_trait]
impl<T: App> Request<T> for CommandRequestFindTorrentByHashId {
    type RequestResult = Option<TorrentToken>;

    async fn request(&mut self, o: &mut T) -> Self::RequestResult {
        o.find_torrent_by_hash_id(&self.0).map(|x| x.token())
    }
}

impl From<&[u8; SHA1_SIZE]> for CommandRequestFindTorrentByHashId {
    fn from(value: &[u8; SHA1_SIZE]) -> Self {
        Self(value.clone())
    }
}
