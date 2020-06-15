use crate::RsbtResult;
use futures::{future::BoxFuture, Future};
use std::time::Duration;

pub trait AppRuntime {
    fn spawn<'a, F>(f: F) -> BoxFuture<'a, RsbtResult<F::Output>>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;

    fn delay_for<'a>(duration: Duration) -> BoxFuture<'a, ()>;

    fn timeout<'a, T>(duration: Duration, future: T) -> BoxFuture<'a, RsbtResult<T::Output>>
    where
        T: Future + Send + 'a;
}
