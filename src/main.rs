use rsbt_draf::{RsbtApp, RsbtTokioApp, RsbtAppProperties};

#[tokio::main]
async fn main() {
    let properties = RsbtAppProperties;

    let app = RsbtTokioApp::new(properties);

    let _ = app.spawn();

    println!("best bittorrent client in the world!");
}
