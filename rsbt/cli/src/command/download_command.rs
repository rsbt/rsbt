use std::path::PathBuf;

use super::{AppError, Parser, Runnable};

/// Downloads torrent(s) to desired target dir or file.
#[derive(Debug, Parser)]
pub struct DownloadCommand {
    /// Torrent(s).
    #[arg(required = true)]
    torrent: Vec<PathBuf>,
    // /// Output dir.
    // #[arg(short, long)]
    // out_dir: PathBuf,
}

impl Runnable for DownloadCommand {
    fn run(self) -> Result<(), AppError> {
        let app = rsbt_app::BlockingApp::default();

        Ok(())
    }
}
