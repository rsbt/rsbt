use super::{AnyRequest, AnyResult};
use async_trait::async_trait;
use futures::future::BoxFuture;

pub struct AnyCommand<A, R: 'static>(
    pub Option<Box<dyn FnOnce(&mut A) -> BoxFuture<'_, R> + Send + Sync>>,
);

#[async_trait]
impl<A, R> AnyRequest<A> for AnyCommand<A, R>
where
    A: Send,
    R: Send + Sync + 'static,
{
    async fn any_request(&mut self, o: &mut A) -> AnyResult {
        if let Some(command) = self.0.take() {
            let result: AnyResult = Box::new(command(o).await);
            result
        } else {
            panic!();
        }
    }
}
