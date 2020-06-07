use crate::{
    commands::Command,
    tasks::{
        AppCommandChannel, AppCommandReceiver, AppCommandSender, AppFactory, AppHandler,
        AppProperties, Receiver, Sender,
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
    type Factory: AppFactory;
    type Properties: AppProperties;

    fn init(
        properties: Self::Properties,
        app_handler: AppHandler<
            <<Self::Factory as AppFactory>::CommandChannel as AppCommandChannel>::CommandSender,
        >,
        command_receiver: <<Self::Factory as AppFactory>::CommandChannel as AppCommandChannel>::CommandReceiver,
    ) -> Self;

    fn new(properties: Self::Properties) -> Self {
        let (command_sender, command_receiver) =
            <<Self::Factory as AppFactory>::CommandChannel as AppCommandChannel>::create();
        Self::init(properties, command_sender.into(), command_receiver)
    }

    fn spawn(mut self) -> BoxFuture<'static, ()> {
        <Self::Factory as AppFactory>::spawn(async move {
            self.run();
        })
    }

    fn properties(&self) -> Arc<Self::Properties>;

    fn app_handler(
        &mut self,
    ) -> &mut AppHandler<
        <<Self::Factory as AppFactory>::CommandChannel as AppCommandChannel>::CommandSender,
    >;

    fn command_receiver(
        &mut self,
    ) -> &mut <<Self::Factory as AppFactory>::CommandChannel as AppCommandChannel>::CommandReceiver;

    async fn run(&mut self) {}
}
