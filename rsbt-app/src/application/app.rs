use crate::{
    application::{
        AppCommandChannel, AppCommandReceiver, AppCommandSender, AppHandler, AppProperties,
        AppRuntime,
    },
    bridge::{OneshotChannel, SocketListener, SocketStream},
    methods::{AnyResult, Method},
    torrent::{TorrentProcess, TorrentProcessStatus, TorrentToken},
    transport::IncomingConnection,
    RsbtResult, SHA1_SIZE,
};
use async_trait::async_trait;
use futures::{
    future::{abortable, join, BoxFuture},
    StreamExt,
};
use log::error;
use std::sync::Arc;

#[async_trait]
pub trait App: sealed::AppPriv + Send + Sized + 'static {
    type CommandChannel: AppCommandChannel<Self>;
    type CommandReceiver: AppCommandReceiver<Self>;
    type CommandSender: AppCommandSender<Self>;
    type Properties: AppProperties;
    type Runtime: AppRuntime;
    type AnyResultOneshotChannel: OneshotChannel<Self, AnyResult>;
    type SocketStream: SocketStream;
    type SocketListener: SocketListener<Self::SocketStream>;
    type IncomingConnection: IncomingConnection<Self>;

    fn init(
        properties: Self::Properties,
        app_handler: AppHandler<Self>,
        command_receiver: Self::CommandReceiver,
    ) -> Self;

    fn new(properties: Self::Properties) -> Self {
        let (command_sender, command_receiver) = Self::CommandChannel::create();
        Self::init(
            properties,
            AppHandler::new(command_sender),
            command_receiver,
        )
    }

    fn spawn<'a>(self) -> BoxFuture<'a, RsbtResult<()>> {
        Self::Runtime::spawn(self.run())
    }

    fn properties(&self) -> Arc<Self::Properties>;

    fn app_handler(&mut self) -> &mut AppHandler<Self>;

    fn command_receiver(&mut self) -> &mut Option<Self::CommandReceiver>;

    async fn run(mut self) {
        if let Some(mut command_receiver) = self.command_receiver().take() {
            let properties = self.properties();
            let app_handler = self.app_handler().clone();
            let incoming_connections_loop =
                Self::IncomingConnection::process(properties, app_handler);
            let (incoming_connections_loop, incoming_connections_abort_handler) =
                abortable(incoming_connections_loop);

            let command_loop = async move {
                while let Some(cmd) = command_receiver.next().await {
                    cmd.exec(&mut self).await;
                    if !self.is_running() {
                        break;
                    }
                }
                incoming_connections_abort_handler.abort();
            };
            let (_, _) = join(command_loop, incoming_connections_loop).await;
        } else {
            panic!("you must set the app command receiver");
        }
    }

    async fn add_torrent(
        &mut self,
        data: Vec<u8>,
        filename: String,
        state: TorrentProcessStatus,
    ) -> RsbtResult<TorrentToken> {
        todo!()
    }

    fn is_running(&self) -> bool;

    async fn quit(&mut self);

    fn find_torrent_by_hash_id(&self, hash_id: &[u8; SHA1_SIZE]) -> Option<&TorrentProcess> {
        todo!()
    }
}

mod sealed {
    #[cfg(test)]
    use crate::tests::TestApp;
    use crate::TokioApp;

    pub trait AppPriv {}

    impl AppPriv for TokioApp {}
    #[cfg(test)]
    impl AppPriv for TestApp {}
}
