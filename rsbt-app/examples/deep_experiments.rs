use env_logger;
use rsbt::{App, RsbtResult, TokioTypeFactory};

#[tokio::main]
async fn main() -> RsbtResult<()> {
    env_logger::init();
    let app: App<TokioTypeFactory> = App::new(Default::default());
    app.run().await;

    Ok(())
}
