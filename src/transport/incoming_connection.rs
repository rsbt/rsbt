use crate::{
    application::AppProperties,
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
                }
            }
            Err(err) => {
                error!("{}", err);
            }
        }
    }
}

mod sealed {
    use crate::{transport::DefaultIncomingConnection, App};

    pub trait IncomingConnectionPriv {}

    impl IncomingConnectionPriv for DefaultIncomingConnection {}
}
