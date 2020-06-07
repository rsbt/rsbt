use crate::tasks::AppCommandChannel;
use futures::{future::BoxFuture, Future};

pub trait AppFactory {
    type CommandChannel: AppCommandChannel;

    fn spawn<F>(f: F) -> BoxFuture<'static, ()>
    where
        F: Future<Output = ()> + Send + 'static;
}
