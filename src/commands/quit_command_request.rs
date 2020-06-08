use crate::{commands::CommandRequest, tasks::App};
use async_trait::async_trait;

#[derive(Debug)]
pub struct QuitCommandRequest;

#[async_trait]
impl<T: App> CommandRequest<T> for QuitCommandRequest {
    async fn request(&mut self, o: &mut T) {
        todo!()
    }
}
