use crate::{
    announce::Announces,
    peer::Peers,
    statistic::Statistics,
    storage::Storage,
    torrent::{TorrentHeader},
};
use std::sync::Arc;

pub struct TorrentProcess {
    header: Arc<TorrentHeader>,
    peers: Peers,
    announces: Announces,
    storage: Storage,
    statistics: Statistics,
}

impl TorrentProcess {
    /*
    pub(crate) fn token(&self) -> TorrentToken {
        self.header.clone().into()
    }
    */
}

#[cfg(test)]
mod tests {
    use super::TorrentProcess;
    use crate::tests_common::size_check;
    use std::sync::Arc;

    #[test]
    fn check_arc_size() {
        size_check::<TorrentProcess>(1);
        size_check::<Arc<String>>(1);
    }
}
