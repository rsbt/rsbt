use async_trait::async_trait;
use futures::stream::Stream;

#[async_trait]
pub trait Receiver<M>: Stream<Item = M> + Unpin + Send + 'static {}
