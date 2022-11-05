/*!
 # rsbt-rt description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
rsbt-rt = "0.1"
```

*/

use std::future::{Future, IntoFuture};

use futures::{Sink, Stream};

pub trait Runtime: RuntimeHandle {
    type Handle: RuntimeHandle + Clone;

    fn handle(&self) -> Self::Handle;

    fn shutdown_background(self);
}

pub trait RuntimeHandle: Send + 'static {
    type JoinHandle<O>: IntoFuture<Output = Result<O, JoinError>>
    where
        O: Send + 'static;

    type MpscSender<T>: Sink<T> + Send + Unpin + Clone
    where
        T: Send + 'static;
    type MpscReceiver<T>: Stream<Item = T> + Send + Unpin
    where
        T: Send + Unpin + 'static;

    fn block_on<F: Future>(&self, future: F) -> F::Output;
    fn spawn<F>(&self, future: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
    fn spawn_current<F>(future: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;

    fn channel<T: Send + Unpin + 'static>(
        buffer: usize,
    ) -> (Self::MpscSender<T>, Self::MpscReceiver<T>);
}

pub enum JoinError {
    #[cfg(feature = "tokio_1")]
    Tokio1(tokio::task::JoinError),
}

#[cfg(feature = "tokio_1")]
mod tokio_runtime;

#[cfg(feature = "tokio_1")]
pub use tokio_runtime::TokioRuntime;

#[cfg(feature = "tokio_1")]
pub type DefaultRuntime = tokio_runtime::TokioRuntime;

#[cfg(not(feature = "tokio_1"))]
compile_error!("You must enable tokio_1 feature, as it is only one supported at the moment");

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Sink is closed.")]
    Sink,
}
