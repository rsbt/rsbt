use crate::methods::{AnyRequest, AnyResult};
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait Request<A>: Send + Sync + Debug {
    type RequestResult;
    async fn request(&mut self, o: &mut A) -> Self::RequestResult;
}

#[async_trait]
impl<R, T, A> AnyRequest<A> for T
where
    A: Send,
    T: Request<A, RequestResult = R>,
    R: Send + Sync + 'static,
{
    async fn any_request(&mut self, o: &mut A) -> AnyResult {
        let typed_result = self.request(o).await;

        Box::new(typed_result)
    }
}
