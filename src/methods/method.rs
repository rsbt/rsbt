use async_trait::async_trait;

#[async_trait]
pub trait Method<T> {
    async fn exec(&mut self, o: &mut T);
}
