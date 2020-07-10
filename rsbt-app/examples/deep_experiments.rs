use env_logger;
use rsbt::RsbtResult;

#[tokio::main]
async fn main() -> RsbtResult<()> {
    env_logger::init();
    rsbt::experiments::deep_experiments::main().await
}
