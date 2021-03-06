/*!
# rsbt description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
rsbt = "0.1"
```

*/
use std::path::PathBuf;

#[macro_use]
mod commands;

pub mod announce;
pub mod application;
pub mod config;
pub mod error;
pub mod peer;
pub mod properties;
pub mod result;
pub mod statistic;
pub mod storage;
pub mod sync;
pub mod types;

pub mod managers;
pub mod tokio;
pub mod torrent;
pub mod transport;

pub use error::RsbtError;
pub use properties::RsbtAppProperties;
pub use result::RsbtResult;
pub use torrent::TorrentProcessStatus;

pub(crate) const SHA1_SIZE: usize = 20;

pub(crate) const BLOCK_SIZE: usize = 1 << 14;

pub(crate) const PEER_ID: [u8; 20] = *b"-rs0001-zzzzxxxxyyyy";

//FIXME: pub(crate) const PEER_MAX_CONNECTIONS: usize = 50;
pub const TORRENTS_TOML: &str = "torrents.toml";

pub const DEFAULT_CHANNEL_BUFFER: usize = 256;

pub(crate) const RSBT_DIR: &str = ".rsbt";

//FIXME: pub(crate) const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(110);

pub fn app_dir(custom_config_dir: Option<PathBuf>) -> PathBuf {
    custom_config_dir
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| ".".into())
}

pub use crate::tokio::{TokioMpscSender, TokioTypeFactory};
pub use application::App;
pub use application::AppService;
pub use commands::{Command, CommandSender};

pub fn default_app_service() -> impl AppService {
    application::DefaultAppService
}

#[cfg(test)]
pub(crate) mod tests_common {
    use std::mem::size_of;

    pub(crate) fn size_check<T>(required: usize) {
        size_check_bytes::<T>(size_of::<usize>() * required);
    }

    pub(crate) fn size_check_bytes<T>(required_bytes: usize) {
        assert_eq!(size_of::<T>(), required_bytes);
    }
}
