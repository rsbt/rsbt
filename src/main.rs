use rsbt_draf::{RsbtApp, RsbtAppProperties, RsbtCommand, RsbtResult, RsbtTokioApp};

#[tokio::main]
async fn main() -> RsbtResult<()> {
    let properties = RsbtAppProperties;

    let mut app = RsbtTokioApp::new(properties);

    let app_handler = app.app_handler();

    // app_handler.send(RsbtCommand::Simple).await?;

    // app.spawn();
    app.run().await;

    println!("best bittorrent client in the world!");

    Ok(())
}
