use crate::any_result::AnyResult;
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait AnyRequest<A>: Send + Sync + Debug {
    async fn any_request(&mut self, o: &mut A) -> AnyResult;
}
