use env_logger;
use futures::future::try_join;
use log::debug;
use rsbt::{App, RsbtAppProperties, RsbtResult, TokioApp};

#[tokio::main]
async fn main() -> RsbtResult<()> {
    env_logger::init();

    let properties = Default::default();

    let mut app = TokioApp::new(properties);

    let mut app_handler = app.app_handler().clone();

    let f1 = app.spawn();
    let f2 = async move {
        debug!("sending quit...");
        let res = app_handler.quit().await;
        debug!("done");
        res
    };

    try_join(f1, f2).await?;

    println!("best bittorrent client in the world!");

    Ok(())
}
