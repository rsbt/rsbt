use futures::future::join;
use rsbt_draf::{App, RsbtAppProperties, RsbtResult, TokioApp};

#[tokio::main]
async fn main() -> RsbtResult<()> {
    let properties = RsbtAppProperties;

    let mut app = TokioApp::new(properties);

    let mut app_handler = app.app_handler().clone();

    let f1 = app.spawn();
    let f2 = async move {
        app_handler.quit().await.ok();
    };

    join(f1, f2).await;

    println!("best bittorrent client in the world!");

    Ok(())
}
