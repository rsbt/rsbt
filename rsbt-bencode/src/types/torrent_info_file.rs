use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TorrentInfoFile {
    pub path: PathBuf,
    pub length: usize,
}
