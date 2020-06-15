use crate::{
    application::App,
    methods::Request,
    torrent::{TorrentProcessStatus, TorrentToken},
};
use async_trait::async_trait;

#[derive(Debug)]
pub struct CommandRequestAddTorrent {
    pub data: Vec<u8>,
    pub filename: String,
    pub state: TorrentProcessStatus,
}

#[async_trait]
impl<T: App> Request<T> for CommandRequestAddTorrent {
    type RequestResult = TorrentToken;

    async fn request(&mut self, o: &mut T) -> Self::RequestResult {
        todo!()
    }
}
