use crate::{
    application::{
        AppCommandChannel, AppCommandReceiver, AppCommandSender, AppHandler, AppProperties,
        AppRuntime, OneshotChannel,
    },
    methods::{AnyResult, Method},
};
use async_trait::async_trait;
use futures::{future::BoxFuture, StreamExt};
use std::sync::Arc;

#[async_trait]
pub trait App: Send + Sized + 'static {
    type CommandChannel: AppCommandChannel<Self>;
    type CommandReceiver: AppCommandReceiver<Self>;
    type CommandSender: AppCommandSender<Self>;
    type Properties: AppProperties;
    type Runtime: AppRuntime;
    type AnyResultOneshotChannel: OneshotChannel<Self, AnyResult>;

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

    fn spawn(self) -> BoxFuture<'static, ()> {
        Self::Runtime::spawn(self.run())
    }

    fn properties(&self) -> Arc<Self::Properties>;

    fn app_handler(&mut self) -> &mut AppHandler<Self>;

    fn command_receiver(&mut self) -> &mut Option<Self::CommandReceiver>;

    async fn run(mut self) {
        if let Some(mut command_receiver) = self.command_receiver().take() {
            let command_loop = async move {
                while let Some(cmd) = command_receiver.next().await {
                    cmd.exec(&mut self).await;
                    if !self.is_running() {
                        break;
                    }
                }
            };
            command_loop.await;
        } else {
            panic!("you must set app command receiver");
        }
    }

    fn is_running(&self) -> bool;

    async fn quit(&mut self);
}
