use crate::{application::App, methods::Request};
use async_trait::async_trait;

#[derive(Debug)]
pub struct CommandRequestQuit;

#[async_trait]
impl<T: App> Request<T> for CommandRequestQuit {
    type RequestResult = ();

    async fn request(&mut self, o: &mut T) -> Self::RequestResult {
        o.quit().await
    }
}
