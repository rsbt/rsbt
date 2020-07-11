use super::AnyResult;
use async_trait::async_trait;

#[async_trait]
pub trait AnyRequest<A> {
    async fn any_request(&mut self, o: &mut A) -> AnyResult;
}
