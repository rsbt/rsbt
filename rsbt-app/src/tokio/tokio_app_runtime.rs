use crate::{application::AppRuntime, RsbtResult};
use futures::{future::BoxFuture, FutureExt};
use std::{future::Future, time::Duration};

pub struct TokioAppRuntime;

impl AppRuntime for TokioAppRuntime {
    fn spawn<'a, F>(f: F) -> BoxFuture<'a, RsbtResult<F::Output>>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        tokio::spawn(f)
            .map(|x| x.map_err(anyhow::Error::from))
            .boxed()
    }

    fn spawn_blocking<'a, F, R>(f: F) -> BoxFuture<'a, RsbtResult<R>>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        tokio::task::spawn_blocking(f)
            .map(|x| x.map_err(anyhow::Error::from))
            .boxed()
    }

    fn sleep<'a>(duration: Duration) -> BoxFuture<'a, ()> {
        tokio::time::delay_for(duration).boxed()
    }

    fn timeout<'a, T>(duration: Duration, future: T) -> BoxFuture<'a, RsbtResult<T::Output>>
    where
        T: Future + Send + 'a,
    {
        tokio::time::timeout(duration, future)
            .map(|x| x.map_err(anyhow::Error::from))
            .boxed()
    }
}
