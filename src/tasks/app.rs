use crate::{
    commands::Command,
    methods::Method,
    tasks::{
        AppCommandChannel, AppCommandReceiver, AppCommandSender, AppHandler, AppProperties,
        AppRuntime, Receiver, Sender,
    },
};
use async_trait::async_trait;
use futures::{future::BoxFuture, StreamExt};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

#[async_trait]
pub trait App: Send + Sized + 'static {
    type CommandChannel: AppCommandChannel<Self>;
    type CommandReceiver: AppCommandReceiver<Self>;
    type CommandSender: AppCommandSender<Self>;
    type Properties: AppProperties;
    type Runtime: AppRuntime;

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

    fn spawn(mut self) -> BoxFuture<'static, ()> {
        Self::Runtime::spawn(async move {
            self.run().await;
        })
    }

    fn properties(&self) -> Arc<Self::Properties>;

    fn app_handler(&mut self) -> &mut AppHandler<Self>;

    fn command_receiver(&mut self) -> &mut Option<Self::CommandReceiver>;

    async fn run(&mut self) {
        if let Some(mut command_receiver) = self.command_receiver().take() {
            let f = async move {
                while let Some(mut cmd) = command_receiver.next().await {
                    cmd.exec(self).await;
                    if !self.is_running() {
                        break;
                    }
                }
            };
            f.await;
        }
    }

    fn is_running(&self) -> bool;

    async fn quit(&mut self);
}
