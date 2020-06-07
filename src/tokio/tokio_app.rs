use crate::{
    tasks::{App, AppCommandReceiver, AppCommandSender, AppHandler, AppProperties, AppRuntime},
    tokio::{
        TokioAppCommandChannel, TokioAppCommandReceiver, TokioAppCommandSender, TokioAppRuntime,
    },
    RsbtAppProperties,
};
use std::sync::Arc;

pub struct TokioApp {
    properties: Arc<RsbtAppProperties>,
    app_handler: AppHandler<TokioAppCommandSender>,
    command_receiver: TokioAppCommandReceiver,
}

impl App for TokioApp {
    type CommandChannel = TokioAppCommandChannel;
    type CommandReceiver = TokioAppCommandReceiver;
    type CommandSender = TokioAppCommandSender;
    type Properties = RsbtAppProperties;
    type Runtime = TokioAppRuntime;

    fn init(
        properties: Self::Properties,
        app_handler: AppHandler<TokioAppCommandSender>,
        command_receiver: TokioAppCommandReceiver,
    ) -> Self {
        Self {
            properties: Arc::new(properties),
            app_handler,
            command_receiver,
        }
    }

    fn properties(&self) -> Arc<Self::Properties> {
        self.properties.clone()
    }

    fn app_handler(&mut self) -> &mut AppHandler<TokioAppCommandSender> {
        &mut self.app_handler
    }

    fn command_receiver(&mut self) -> &mut TokioAppCommandReceiver {
        &mut self.command_receiver
    }
}
