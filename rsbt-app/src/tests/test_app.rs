use crate::{
    manager::DefaultTorrentManager,
    methods::AnyResult,
    tests::{
        TestAppCommandChannel, TestAppCommandReceiver, TestAppCommandSender, TestAppRuntime,
        TestOneshotChannel, TestSocketListener, TestSocketStream,
    },
    transport::DefaultIncomingConnection,
    App, AppHandler, RsbtAppProperties,
};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug)]
pub struct TestApp {}

#[async_trait]
impl App for TestApp {
    type CommandChannel = TestAppCommandChannel;
    type CommandReceiver = TestAppCommandReceiver;
    type CommandSender = TestAppCommandSender;
    type Properties = RsbtAppProperties;
    type Runtime = TestAppRuntime;
    type AnyResultOneshotChannel = TestOneshotChannel<AnyResult, Self>;
    type SocketStream = TestSocketStream;
    type SocketListener = TestSocketListener;
    type IncomingConnection = DefaultIncomingConnection;
    type TorrentManager = DefaultTorrentManager;
    fn init(
        properties: Self::Properties,
        app_handler: crate::AppHandler<Self>,
        command_receiver: Self::CommandReceiver,
    ) -> Self {
        todo!()
    }
    fn properties(&self) -> Arc<Self::Properties> {
        todo!()
    }
    fn app_handler(&mut self) -> &mut AppHandler<Self> {
        todo!()
    }
    fn command_receiver(&mut self) -> &mut Option<Self::CommandReceiver> {
        todo!()
    }
    fn is_running(&self) -> bool {
        todo!()
    }
    async fn quit(&mut self) {
        todo!()
    }
    fn torrent_manager(&mut self) -> &mut Self::TorrentManager {
        todo!()
    }
}
