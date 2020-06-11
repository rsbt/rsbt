use async_trait::async_trait;

#[async_trait]
pub trait Method<T> {
    async fn exec(self, o: &mut T);
}
