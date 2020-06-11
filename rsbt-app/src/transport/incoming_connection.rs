use crate::{
    application::{AppProperties, AppRuntime},
    bridge::{OneshotChannel, SocketListener, SocketStream},
    App, AppHandler,
};
use async_trait::async_trait;
use futures::{
    future::{join, BoxFuture},
    StreamExt,
};
use log::{debug, error};
use std::sync::Arc;

#[async_trait]
pub trait IncomingConnection<A: App>: sealed::IncomingConnectionPriv {
    async fn process(properties: Arc<A::Properties>, app_handler: AppHandler<A>) {
        match A::SocketListener::bind(*properties.listen_addr()).await {
            Ok(mut listener) => {
                while let Some(socket) = listener.next().await {
                    debug!("peer connection attempted...");
                    match socket {
                        Ok(socket) => {
                            let app_handler = app_handler.clone();
                            A::Runtime::spawn(async move {
                                debug!("process incoming connection");
                                if let Err(err) =
                                    sealed::process_incoming_connection(socket, app_handler).await
                                {
                                    error!("peer connection attempt processing failed: {}", err);
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
                error!("{}", err);
                // FIXME: ask app about it is running
                // if running - sleep retry interval
            }
        }
    }
}

mod sealed {
    use crate::{
        bridge::SocketStream, transport::DefaultIncomingConnection, App, AppHandler, RsbtResult,
    };
    use futures::AsyncReadExt;
    use log::debug;
    use rsbt_bencode::Handshake;
    use std::convert::TryInto;

    pub async fn process_incoming_connection<S: SocketStream, A: App>(
        mut socket: S,
        app_handler: AppHandler<A>,
    ) -> RsbtResult<()> {
        let mut incoming_handshake = vec![0u8; 68];

        debug!("read incoming handshake");

        socket.read_exact(&mut incoming_handshake).await?;

        let handshake: Handshake = incoming_handshake.try_into()?;

        debug!("done...");

        todo!();

        Ok(())
    }

    pub trait IncomingConnectionPriv {}

    impl IncomingConnectionPriv for DefaultIncomingConnection {}
}
