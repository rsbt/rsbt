use crate::{application::App, methods::Request};
use async_trait::async_trait;
use futures::future::BoxFuture;
use std::fmt::Debug;

#[derive(Debug)]
pub struct CommandRequestQuit;

#[async_trait]
impl<T: App> Request<T> for CommandRequestQuit {
    type RequestResult = ();

    async fn request(&mut self, o: &mut T) -> Self::RequestResult {
        o.quit().await
    }
}

pub struct CommandRequestAny<T: App, R>(
    pub Option<Box<dyn FnOnce(&mut T) -> BoxFuture<'_, R> + Send + Sync>>,
);

impl<T: App, R: Debug> Debug for CommandRequestAny<T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[async_trait]
impl<T: App, R: Debug> Request<T> for CommandRequestAny<T, R> {
    type RequestResult = R;

    async fn request(&mut self, o: &mut T) -> Self::RequestResult {
        self.0.take().unwrap()(o).await
    }
}
