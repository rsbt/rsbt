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

    fn delay_for(duration: Duration) -> LocalBoxFuture<'static, ()> {
        tokio::time::delay_for(duration).boxed_local()
    }

    fn timeout<T>(duration: Duration, future: T) -> LocalBoxFuture<'static, RsbtResult<T::Output>>
    where
        T: Future + 'static,
    {
        tokio::time::timeout(duration, future)
            .map(|x| x.map_err(anyhow::Error::from))
            .boxed_local()
    }
}
