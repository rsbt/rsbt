use crate::RsbtResult;
use futures::{
    future::{BoxFuture, LocalBoxFuture},
    Future,
};
use std::time::Duration;

pub trait AppRuntime {
    fn spawn<F>(f: F) -> BoxFuture<'static, RsbtResult<F::Output>>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;

    fn delay_for(duration: Duration) -> LocalBoxFuture<'static, ()>;
    fn timeout<T>(duration: Duration, future: T) -> LocalBoxFuture<'static, RsbtResult<T::Output>>
    where
        T: Future + 'static;
}
