use crate::{any_result::AnyResult, tasks::App};
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait CommandRequest<A: App>: Send + Sync + Debug {
    async fn request(&mut self, o: &mut A) -> AnyResult;
}

#[async_trait]
pub trait CommandRequestTyped<A: App>: Send + Sync + Debug {
    type RequestResult;
    async fn request_typed(&mut self, o: &mut A) -> Self::RequestResult;
}

#[async_trait]
impl<R, T, A> CommandRequest<A> for T
where
    A: App,
    T: CommandRequestTyped<A, RequestResult = R>,
    R: Send + Sync + 'static,
{
    async fn request(&mut self, o: &mut A) -> AnyResult {
        let typed_result = self.request_typed(o).await;
        Box::new(typed_result)
    }
}
