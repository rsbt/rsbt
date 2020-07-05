use env_logger;
use futures::future::try_join;
use log::{debug, error};
use rsbt::{App, RsbtAppProperties, RsbtResult, TokioApp, TorrentProcessStatus};
use std::time::Duration;

#[tokio::main]
async fn main() -> RsbtResult<()> {
    env_logger::init();
    rsbt::experiments::deep_experiments::main().await
}
