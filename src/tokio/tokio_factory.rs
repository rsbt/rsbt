use crate::{tasks::AppFactory, tokio::TokioAppCommandChannel};
use futures::future::BoxFuture;

pub struct TokioFactory;

impl AppFactory for TokioFactory {
    type CommandChannel = TokioAppCommandChannel;
    fn spawn<F>(f: F) -> BoxFuture<'static, ()>
    where
        F: futures::Future<Output = ()> + Send + 'static,
    {
        Box::pin(async move {
            tokio::spawn(f);
        })
    }
}
