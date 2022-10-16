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

use std::{
    future::{Future, IntoFuture},
    marker::PhantomData,
    pin::Pin,
};

pub trait Runtime {
    type JoinHandle: RuntimeJoinHandle + Send + 'static;

    fn block_on<F: Future>(&self, future: F) -> F::Output;
    fn spawn<F>(&self, future: F) -> JoinHandle<Self>
    where
        F: Future<Output = ()> + Send + 'static;
}

pub trait RuntimeJoinHandle {
    type Output;
}

pub struct JoinHandle<R: Runtime + ?Sized> {
    inner: <R as Runtime>::JoinHandle,
    _r: PhantomData<R>,
}

impl<R: Runtime + ?Sized> JoinHandle<R> {
    pub fn new(inner: <R as Runtime>::JoinHandle) -> Self {
        Self {
            inner,
            _r: PhantomData,
        }
    }
}

impl<R: Runtime + ?Sized, T, E> IntoFuture for JoinHandle<R>
where
    R::JoinHandle: IntoFuture<Output = Result<T, E>>,
    <R::JoinHandle as IntoFuture>::IntoFuture: Send + 'static,
{
    type Output = Result<T, E>;

    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'static>>;

    fn into_future(self) -> Self::IntoFuture {
        use futures::TryFutureExt;
        Box::pin(self.inner.into_future().map_err(|x| x))
    }
}

enum JoinError {
    #[cfg(feature = "tokio_1")]
    Tokio1(tokio::task::JoinError)
}

#[cfg(feature = "tokio_1")]
mod tokio_runtime;

#[cfg(feature = "tokio_1")]
pub use tokio_runtime::TokioRuntime;

#[cfg(feature = "tokio_1")]
pub type DefaultRuntime = tokio_runtime::TokioRuntime;

#[cfg(not(feature = "tokio_1"))]
compile_error!("You must enable tokio_1 feature, as it is only one supported in the moment");
