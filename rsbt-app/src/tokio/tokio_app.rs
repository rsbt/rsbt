use crate::{
    application::{App, AppHandler},
    experiments::Action,
    manager::DefaultTorrentsManager,
    methods::AnyResult,
    tokio::{
        TokioAppCommandChannel, TokioAppCommandReceiver, TokioAppCommandSender, TokioAppRuntime,
        TokioOneshotChannel, TokioReceiver, TokioSocketListener, TokioSocketStream,
    },
    transport::DefaultIncomingConnection,
    RsbtAppProperties,
};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug)]
pub struct TokioApp {
    running: bool,
    properties: Arc<RsbtAppProperties>,
    app_handler: AppHandler<TokioApp>,
    command_receiver: Option<TokioAppCommandReceiver>,
    torrent_manager: DefaultTorrentsManager,
}

#[async_trait]
impl App for TokioApp {
    type CommandChannel = TokioAppCommandChannel;
    type CommandReceiver = TokioAppCommandReceiver;
    type CommandSender = TokioAppCommandSender;
    type Properties = RsbtAppProperties;
    type Runtime = TokioAppRuntime;
    type AnyResultOneshotChannel = TokioOneshotChannel<AnyResult>;
    type SocketStream = TokioSocketStream;
    type SocketListener = TokioSocketListener;
    type IncomingConnection = DefaultIncomingConnection;
    type TorrentManager = DefaultTorrentsManager;

    fn init(
        properties: Self::Properties,
        app_handler: AppHandler<TokioApp>,
        command_receiver: TokioAppCommandReceiver,
    ) -> Self {
        Self {
            running: true,
            properties: Arc::new(properties),
            app_handler,
            command_receiver: Some(command_receiver),
            torrent_manager: DefaultTorrentsManager,
        }
    }

    fn properties(&self) -> Arc<Self::Properties> {
        self.properties.clone()
    }

    fn app_handler(&mut self) -> &mut AppHandler<TokioApp> {
        &mut self.app_handler
    }

    fn command_receiver(&mut self) -> &mut Option<TokioAppCommandReceiver> {
        &mut self.command_receiver
    }

    fn is_running(&self) -> bool {
        self.running
    }

    async fn quit(&mut self) {
        self.running = false;
    }
    fn torrent_manager(&mut self) -> &mut Self::TorrentManager {
        &mut self.torrent_manager
    }
}
