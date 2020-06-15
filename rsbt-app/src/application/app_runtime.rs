use crate::RsbtResult;
use futures::{future::BoxFuture, Future};
use std::time::Duration;

pub trait AppRuntime {
    fn spawn<'a, F>(f: F) -> BoxFuture<'a, RsbtResult<F::Output>>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;

    fn delay_for(duration: Duration) -> BoxFuture<'static, ()>;
    fn timeout<T>(duration: Duration, future: T) -> BoxFuture<'static, RsbtResult<T::Output>>
    where
        T: Future + Send + 'static;
}
