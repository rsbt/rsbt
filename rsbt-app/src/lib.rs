mod app;
mod blocking_app;
mod error;
mod message;
mod status;

use std::path::PathBuf;

pub use rsbt_rt::DefaultRuntime;

pub use crate::{
    app::App,
    blocking_app::{BlockingApp, BlockingAppBuilder},
    error::AppError,
    message::Message,
    status::AppStatus,
};

pub trait Actor {
    type Handle: ActorHandle;
}

pub trait ActorHandle {}

pub trait Input {}

pub trait Output {}

pub struct Download<I: Input, O: Output> {
    input: I,
    output: O,
}

impl<I: Input, O: Output> Download<I, O> {
    pub fn new(input: I, output: O) -> Self {
        Self { input, output }
    }
}

impl<I: Input, O: Output> Actor for Download<I, O> {
    type Handle = DownloadHandle;
}

pub struct DownloadHandle {}

impl ActorHandle for DownloadHandle {}

pub struct DefaultFileOutput(PathBuf);

impl From<PathBuf> for DefaultFileOutput {
    fn from(value: PathBuf) -> Self {
        Self(value)
    }
}

impl Output for DefaultFileOutput {}

pub struct PathBufInput(pub PathBuf);

impl Input for PathBufInput {}
