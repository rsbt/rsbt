use crate::{
    commands::Command,
    tasks::{
        AppCommandChannel, AppCommandReceiver, AppCommandSender, AppHandler, AppProperties,
        AppRuntime, Receiver, Sender,
    },
};
use async_trait::async_trait;
use futures::future::BoxFuture;
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

#[async_trait]
pub trait App: Send + Sized + 'static {
    type CommandChannel: AppCommandChannel<Self::CommandSender, Self::CommandReceiver>;
    type CommandReceiver: AppCommandReceiver;
    type CommandSender: AppCommandSender;
    type Properties: AppProperties;
    type Runtime: AppRuntime;

    fn init(
        properties: Self::Properties,
        app_handler: AppHandler<Self::CommandSender>,
        command_receiver: Self::CommandReceiver,
    ) -> Self;

    fn new(properties: Self::Properties) -> Self {
        let (command_sender, command_receiver) = Self::CommandChannel::create();
        Self::init(properties, command_sender.into(), command_receiver)
    }

    fn spawn(mut self) -> BoxFuture<'static, ()> {
        Self::Runtime::spawn(async move {
            self.run().await;
        })
    }

    fn properties(&self) -> Arc<Self::Properties>;

    fn app_handler(&mut self) -> &mut AppHandler<Self::CommandSender>;

    fn command_receiver(&mut self) -> &mut Self::CommandReceiver;

    async fn run(&mut self) {}
}
