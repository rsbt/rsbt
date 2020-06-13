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

mod announce;
mod application;
mod bridge;
mod commands;
mod error;
mod methods;
mod peer;
mod properties;
mod result;
mod statistic;
mod storage;
mod sync;
mod types;

mod manager;
#[cfg(test)]
mod tests;
mod tokio;
mod torrent;
mod transport;

pub use application::App;

pub use crate::tokio::TokioApp;
pub use application::AppHandler;
pub use commands::Command;
pub use error::RsbtError;
pub use properties::RsbtAppProperties;
pub use result::RsbtResult;

pub(crate) const SHA1_SIZE: usize = 20;

pub(crate) const BLOCK_SIZE: usize = 1 << 14;

pub(crate) const PEER_ID: [u8; 20] = *b"-rs0001-zzzzxxxxyyyy";

//FIXME: pub(crate) const PEER_MAX_CONNECTIONS: usize = 50;
pub const TORRENTS_TOML: &str = "torrents.toml";

pub const DEFAULT_CHANNEL_BUFFER: usize = 256;

//FIXME: pub(crate) const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(110);

pub(crate) fn count_parts(total: usize, part_size: usize) -> usize {
    total / part_size + if total % part_size != 0 { 1 } else { 0 }
}

pub fn default_app_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(".rsbt")
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
