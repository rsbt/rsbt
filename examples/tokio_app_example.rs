use env_logger;
use futures::future::join;
use rsbt::{App, RsbtAppProperties, RsbtResult, TokioApp};

#[tokio::main]
async fn main() -> RsbtResult<()> {
    env_logger::init();
    let properties = Default::default();

    let app = TokioApp::new(properties);

    // let mut app_handler = app.app_handler().clone();
    /*
        let f1 = app.spawn();
        let f2 = async move {
            app_handler.quit().await.ok();
        };

        join(f1, f2).await;
    */
    app.run().await;
    println!("best bittorrent client in the world!");

    Ok(())
}
