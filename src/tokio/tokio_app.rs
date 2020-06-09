use crate::{
    methods::AnyResult,
    application::{App, AppHandler},
    tokio::{
        TokioAppCommandChannel, TokioAppCommandReceiver, TokioAppCommandSender, TokioAppRuntime,
        TokioOneshotChannel,
    },
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
}

#[async_trait]
impl App for TokioApp {
    type CommandChannel = TokioAppCommandChannel;
    type CommandReceiver = TokioAppCommandReceiver;
    type CommandSender = TokioAppCommandSender;
    type Properties = RsbtAppProperties;
    type Runtime = TokioAppRuntime;
    type AnyResultOneshotChannel = TokioOneshotChannel<AnyResult>;

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
}
