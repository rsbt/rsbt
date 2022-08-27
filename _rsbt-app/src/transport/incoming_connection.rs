use async_trait::async_trait;
use futures::{
    future::{join, BoxFuture},
    StreamExt,
};
use log::{debug, error, info};
use std::{sync::Arc, time::Duration};

pub struct IncomingConnection;
/*
#[async_trait]
pub trait IncomingConnection<A: App>: private::IncomingConnectionPriv {
    async fn process(properties: Arc<A::Properties>, app_handler: AppHandler<A>) {
        let listen_addr = *properties.listen_addr();

        info!("incoming connection listener on {}", listen_addr);

        loop {
            match A::SocketListener::bind(listen_addr).await {
                Ok(mut listener) => {
                    while let Some(socket) = listener.next().await {
                        debug!("peer connection attempted...");
                        match socket {
                            Ok(socket) => {
                                let app_handler = app_handler.clone();
                                A::Runtime::spawn(async move {
                                    debug!("process incoming connection");
                                    if let Err(err) =
                                        private::process_incoming_connection(socket, app_handler)
                                            .await
                                    {
                                        error!(
                                            "peer connection attempt processing failed: {}",
                                            err
                                        );
                                    }
                                });
                            }
                            Err(err) => {
                                // FIXME: need to check which class of errors come here
                                error!("peer connection attempt failed: {}", err);
                            }
                        }
                    }
                }
                Err(err) => {
                    error!("cannot start listener: {}", err);
                    // FIXME: ask app about it is running
                    // if running - sleep retry interval
                    error!("retry in 5 seconds...");
                    A::Runtime::delay_for(Duration::from_secs(5)).await;
                }
            }
        }
    }
}

mod private {
    use crate::{
        bridge::SocketStream, transport::DefaultIncomingConnection, App, AppHandler, RsbtResult,
    };
    use futures::{AsyncReadExt, AsyncWriteExt};
    use log::debug;
    use rsbt_bencode::Handshake;
    use std::convert::TryInto;

    pub async fn process_incoming_connection<S: SocketStream, A: App>(
        mut socket: S,
        mut app_handler: AppHandler<A>,
    ) -> RsbtResult<()> {
        let mut incoming_handshake = vec![0u8; 68];

        debug!("reading incoming handshake...");

        socket.read_exact(&mut incoming_handshake).await?;

        debug!("handshake received");

        debug!("parsing handshake...");

        let handshake: Handshake = incoming_handshake.try_into()?;

        debug!("handshake parsed");

        debug!("finding a torrent process by hash id...");
        if let Some(mut torrent_token) = app_handler
            .find_torrent_by_hash_id(handshake.info_hash)
            .await?
        {
            socket.write_all(torrent_token.handshake()).await?;

            torrent_token.accept_peer_connection(socket).await?;
        } else {
            debug!("torrent not found, closing connection");
        }

        Ok(())
    }

    pub trait IncomingConnectionPriv {}

    impl IncomingConnectionPriv for DefaultIncomingConnection {}
}
*/
