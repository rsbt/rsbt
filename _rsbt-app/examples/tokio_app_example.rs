/*
use env_logger;
use futures::future::try_join;
use log::{debug, error};
use rsbt::{App, RsbtAppProperties, RsbtResult, TokioApp, TorrentProcessStatus};
use std::time::Duration;

#[tokio::main]
async fn main() -> RsbtResult<()> {
    env_logger::init();

    let properties = Default::default();

    let mut app = TokioApp::new(properties);

    let mut app_handler = app.app_handler().clone();

    let f1 = app.spawn();
    let f2 = async move {
        debug!("loading big-buck-bunny.torrent");

        match app_handler
            .add_torrent(
                include_bytes!("../../rsbt-bencode/tests/big-buck-bunny.torrent").to_vec(),
                "big-buck-bunny.torrent".into(),
                TorrentProcessStatus::Enabled,
            )
            .await
        {
            Ok(_torrent_token) => {
                debug!("added torrent...");
            }
            Err(err) => error!("cannot add torrent: {}", err),
        }

        debug!("waiting 10 seconds...");
        tokio::time::delay_for(Duration::from_secs(10)).await;

        debug!("sending quit...");
        let res = app_handler.quit().await;
        debug!("done");
        res
    };

    try_join(f1, f2).await?;

    println!("best bittorrent client in the world!");

    Ok(())
}
*/
fn main() {}
