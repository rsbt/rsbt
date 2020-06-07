use futures::{future::BoxFuture, Future};

pub trait AppRuntime {
    fn spawn<F>(f: F) -> BoxFuture<'static, ()>
    where
        F: Future<Output = ()> + Send + 'static;
}
