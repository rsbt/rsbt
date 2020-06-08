use crate::tasks::App;
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait CommandRequest<A: App>: Send + Debug {
    async fn request(&mut self, o: &mut A);
}
