use crate::{methods::Request, application::App};
use async_trait::async_trait;

#[derive(Debug)]
pub struct QuitCommandRequest;

#[async_trait]
impl<T: App> Request<T> for QuitCommandRequest {
    type RequestResult = ();

    async fn request(&mut self, o: &mut T) -> Self::RequestResult {
        o.quit().await
    }
}
