use crate::{application::AppRuntime, RsbtResult};
use futures::{
    future::{BoxFuture, LocalBoxFuture},
    Future, FutureExt,
};
use std::time::Duration;

pub struct TokioAppRuntime;

impl AppRuntime for TokioAppRuntime {
    fn spawn<F>(f: F) -> BoxFuture<'static, RsbtResult<F::Output>>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        tokio::spawn(f)
            .map(|x| x.map_err(anyhow::Error::from))
            .boxed()
    }

    fn delay_for(duration: Duration) -> BoxFuture<'static, ()> {
        tokio::time::delay_for(duration).boxed()
    }

    fn timeout<T>(duration: Duration, future: T) -> BoxFuture<'static, RsbtResult<T::Output>>
    where
        T: Future + Send + 'static,
    {
        tokio::time::timeout(duration, future)
            .map(|x| x.map_err(anyhow::Error::from))
            .boxed()
    }
}
