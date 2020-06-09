use async_trait::async_trait;

#[async_trait]
pub trait Sender<M, R>: Clone + Send + 'static {
    async fn send(&mut self, m: M) -> R;
}
