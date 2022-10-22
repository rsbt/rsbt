use std::path::PathBuf;

use rsbt_app::{DefaultFileOutput, DefaultRuntime, Download, PathBufInput};

use super::{AppError, Parser, Runnable};

/// Downloads torrent(s) to desired target dir or file.
#[derive(Debug, Parser)]
pub struct DownloadCommand {
    /// Torrent(s).
    #[arg(required = true)]
    torrents: Vec<PathBuf>,
    /// Output dir.
    #[arg(short, long)]
    out_dir: PathBuf,
}

impl Runnable for DownloadCommand {
    fn run(self) -> Result<(), AppError> {
        let app = rsbt_app::BlockingApp::builder()
            .runtime(DefaultRuntime::new().map_err(AppError::Runtime)?)
            .build();

        let mut message_receiver = app.message_receiver();

        for torrent_download in self.torrents.into_iter().map(|x| {
            Download::new(
                PathBufInput(x),
                DefaultFileOutput::from(self.out_dir.clone()),
            )
        }) {
            let handler = app.start(torrent_download)?;
            message_receiver.subscribe(handler)?;
        }

        while let Some(message) = message_receiver.next().transpose()? {}

        app.shutdown()?;

        Ok(())
    }
}
