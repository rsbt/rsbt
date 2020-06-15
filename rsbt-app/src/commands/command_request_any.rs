use crate::{application::App, methods::Request};
use async_trait::async_trait;
use futures::future::BoxFuture;
use std::fmt::Debug;

pub struct CommandRequestAny<T: App, R>(
    pub Option<Box<dyn FnOnce(&mut T) -> BoxFuture<'_, R> + Send + Sync>>,
);

impl<T: App, R> Debug for CommandRequestAny<T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandRequestAny")
    }
}

#[async_trait]
impl<T: App, R> Request<T> for CommandRequestAny<T, R> {
    type RequestResult = R;

    async fn request(&mut self, o: &mut T) -> Self::RequestResult {
        if let Some(command) = self.0.take() {
            command(o).await
        } else {
            panic!("cannot call same command twice")
        }
    }
}
