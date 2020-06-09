use crate::application::AppRuntime;
use futures::future::BoxFuture;

pub struct TokioAppRuntime;

impl AppRuntime for TokioAppRuntime {
    fn spawn<F>(f: F) -> BoxFuture<'static, ()>
    where
        F: futures::Future<Output = ()> + Send + 'static,
    {
        Box::pin(async move {
            tokio::spawn(f);
        })
    }
}
