use futures::stream::Stream;

pub trait Receiver<M>: Stream<Item = M> + Unpin + Send + 'static {}
