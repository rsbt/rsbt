use env_logger;
use rsbt_app::{App, RsbtResult, TokioTypeFactory};

#[tokio::main]
async fn main() -> RsbtResult<()> {
    env_logger::init();

    let app = App::<TokioTypeFactory>::new(Default::default());

    app.run().await;

    Ok(())
}
