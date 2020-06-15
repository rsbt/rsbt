pub struct TorrentHeader {
    id: usize,
}

impl TorrentHeader {
    pub fn id(&self) -> usize {
        self.id
    }
}
