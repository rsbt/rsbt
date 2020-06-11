use crate::{application::AppRuntime, RsbtResult};
use futures::{future::BoxFuture, Future, FutureExt};

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
}
