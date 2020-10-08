use crate::RsbtResult;
use futures::future::BoxFuture;
use std::{future::Future, time::Duration};

pub trait AppRuntime {
    fn spawn<'a, F>(f: F) -> BoxFuture<'a, RsbtResult<F::Output>>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;

    fn spawn_blocking<'a, F, R>(f: F) -> BoxFuture<'a, RsbtResult<R>>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static;

    fn delay_for<'a>(duration: Duration) -> BoxFuture<'a, ()>;

    fn timeout<'a, T>(duration: Duration, future: T) -> BoxFuture<'a, RsbtResult<T::Output>>
    where
        T: Future + Send + 'a;
}
