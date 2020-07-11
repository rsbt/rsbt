use crate::AppService;
use async_trait::async_trait;

pub struct DefaultAppService;

#[async_trait]
impl AppService for DefaultAppService {
    async fn start() -> crate::RsbtResult<()> {
        todo!()
    }

    async fn stop() -> crate::RsbtResult<()> {
        todo!()
    }

    async fn quit() -> crate::RsbtResult<()> {
        todo!()
    }
}
