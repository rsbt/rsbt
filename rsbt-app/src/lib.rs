mod error;
mod status;

use std::path::PathBuf;

use rsbt_types::Torrent;

pub use crate::error::AppError;
pub use crate::status::AppStatus;

pub trait Output {}

pub trait Runtime {}

#[derive(Default)]
pub struct App {}

#[derive(Default)]
pub struct BlockingApp<O: Output, R: Runtime> {
    app: App,
    output: O,
    runtime: R,
}

impl<O, R> BlockingApp<O, R>
where
    O: Output,
    R: Runtime,
{
    pub fn builder() -> BlockingAppBuilder<O, R> {
        BlockingAppBuilder {
            output: None,
            runtime: None,
        }
    }

    pub fn download<'item, I>(self, items: I)
    where
        I: IntoIterator<Item = Torrent<'item>>,
    {
        todo!()
    }
}

pub struct BlockingAppBuilder<O: Output, R: Runtime> {
    output: Option<O>,
    runtime: Option<R>,
}

impl<O, R> BlockingAppBuilder<O, R>
where
    O: Output,
    R: Runtime,
{
    pub fn output(mut self, output: O) -> Self {
        self.output = Some(output);
        self
    }

    pub fn runtime(mut self, runtime: R) -> Self {
        self.runtime = Some(runtime);
        self
    }

    pub fn build(self) -> BlockingApp<O, R> {
        BlockingApp {
            app: App {},
            output: self.output.expect("output"),
            runtime: self.runtime.expect("runtime"),
        }
    }
}

pub struct DefaultFileOutput(PathBuf);

impl From<PathBuf> for DefaultFileOutput {
    fn from(value: PathBuf) -> Self {
        Self(value)
    }
}

impl Output for DefaultFileOutput {}

pub struct DefaultRuntime;

impl Runtime for DefaultRuntime {}
