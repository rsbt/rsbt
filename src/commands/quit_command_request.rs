use crate::{any_result::AnyResult, commands::CommandRequest, tasks::App};
use async_trait::async_trait;
use std::any::Any;
use super::command_request::CommandRequestTyped;

#[derive(Debug)]
pub struct QuitCommandRequest;

#[async_trait]
impl<T: App> CommandRequestTyped<T> for QuitCommandRequest {
    type RequestResult = ();

    async fn request_typed(&mut self, o: &mut T) -> Self::RequestResult {
        o.quit().await
    }
}
