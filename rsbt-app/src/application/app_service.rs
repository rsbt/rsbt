use super::AppTypeFactory;
use crate::RsbtResult;
use async_trait::async_trait;

#[async_trait]
pub trait AppService {
    async fn start() -> RsbtResult<()>;
    async fn stop() -> RsbtResult<()>;
    async fn quit() -> RsbtResult<()>;
}
