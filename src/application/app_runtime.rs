use crate::RsbtResult;
use futures::{future::BoxFuture, Future};

pub trait AppRuntime {
    fn spawn<F>(f: F) -> BoxFuture<'static, RsbtResult<F::Output>>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
}
