use rsbt_draf::{RsbtApp, RsbtTokioApp};

#[tokio::main]
async fn main() {
    let mut app = RsbtTokioApp;

    app.run().await;

    println!("best bittorrent client in the world!");
}
