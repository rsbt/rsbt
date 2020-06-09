use crate::{bridge::Sender, RsbtResult};
use async_trait::async_trait;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
pub struct TestSender<M>(PhantomData<M>);

impl<M> Clone for TestSender<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[async_trait]
impl<M: Debug + Sync + Send + 'static> Sender<M, RsbtResult<()>> for TestSender<M> {
    async fn send(&mut self, m: M) -> RsbtResult<()> {
        todo!()
    }
}
