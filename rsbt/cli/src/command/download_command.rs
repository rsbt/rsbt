use std::path::PathBuf;

use rsbt_app::{DefaultFileOutput, DefaultRuntimeBuilder, Download, PathBufInput, TorrentEvent};

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
            .runtime(DefaultRuntimeBuilder::build()?)
            .build();

        let message_channel = app.message_channel::<TorrentEvent>();

        let mut handles = vec![];
        for torrent_download in self.torrents.into_iter().map(|x| {
            Download::new(
                PathBufInput(x),
                DefaultFileOutput::from(self.out_dir.clone()),
            )
        }) {
            let mut handle = app.start(torrent_download);
            message_channel.subscribe(&mut handle)?;
            handles.push(handle);
        }

        let mut message_receiver = message_channel.listen();

        while let Some(torrent_event) = message_receiver.next() {}

        app.shutdown()?;

        Ok(())
    }
}
