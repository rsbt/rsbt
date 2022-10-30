mod actor;
mod app;
mod blocking_app;
mod error;
mod status;
mod torrent_event;

use std::path::PathBuf;

pub use rsbt_rt::DefaultRuntime;
use rsbt_rt::{Runtime, RuntimeHandle};

pub use crate::{
    actor::{Download, DownloadHandle},
    app::App,
    blocking_app::{BlockingApp, BlockingAppBuilder},
    error::AppError,
    status::AppStatus,
    torrent_event::TorrentEvent,
};

pub trait Actor {
    // type Message;
    type Handle: ActorHandle;

    // fn handle_message(&mut self, msg: Self::Message);
}

pub trait ActorHandle {}

// pub struct ActorEventLoop<A: Actor, R: RuntimeHandle<DefaultTypeMap>> {
// actor: A,
// receiver:
// }

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
