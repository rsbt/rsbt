use futures::future::BoxFuture;

pub trait Sender<T> {
    type Error;

    fn send(&self, value: T) -> BoxFuture<Result<(), Self::Error>>;
}
