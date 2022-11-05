mod actor;
mod app;
mod blocking_app;
mod error;
mod status;
mod torrent_event;

use std::path::PathBuf;

pub use crate::{
    actor::{Actor, ActorHandle, Download, DownloadHandle},
    app::App,
    blocking_app::{BlockingApp, BlockingAppBuilder},
    error::AppError,
    status::AppStatus,
    torrent_event::TorrentEvent,
};

pub trait Input {}

pub trait Output {}

pub struct DefaultFileOutput(PathBuf);

impl From<PathBuf> for DefaultFileOutput {
    fn from(value: PathBuf) -> Self {
        Self(value)
    }
}

impl Output for DefaultFileOutput {}

pub struct PathBufInput(pub PathBuf);

impl Input for PathBufInput {}

pub(crate) mod tokio {
    pub type Runtime = tokio::runtime::Runtime;
    pub type Handle = tokio::runtime::Handle;
    pub type MpscSender<T> = tokio::sync::mpsc::Sender<T>;
    pub type MpscReceiver<T> = tokio::sync::mpsc::Receiver<T>;
    pub use tokio::spawn;
    pub use tokio::sync::mpsc::channel as mpsc_channel;
}

pub struct DefaultRuntimeBuilder;

impl DefaultRuntimeBuilder {
    pub fn build() -> Result<tokio::Runtime, AppError> {
        ::tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(AppError::RuntimeInit)
    }
}
