use rsbt_draf::{App, AppHandler, Command, RsbtAppProperties, RsbtResult, TokioApp};

#[tokio::main]
async fn main() -> RsbtResult<()> {
    let properties = RsbtAppProperties;

    let mut app = TokioApp::new(properties);

    let mut app_handler = app.app_handler().clone();

    app.spawn();

    app_handler.quit().await?;

    // app_handler.send(RsbtCommand::Simple).await?;

    // app.run().await;

    println!("best bittorrent client in the world!");

    Ok(())
}
