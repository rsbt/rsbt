use crate::{
    commands::Command,
    tasks::{
        AppCommandChannel, AppCommandReceiver, AppCommandSender, AppFactory, AppHandler, Receiver,
        Sender,
    },
};
use async_trait::async_trait;
use futures::future::BoxFuture;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[async_trait]
pub trait App {
    type Factory: AppFactory;

    async fn run(
        &mut self,
    ) -> AppHandler<
        <<Self::Factory as AppFactory>::CommandChannel as AppCommandChannel>::CommandSender,
    > {
        let (command_sender, _command_receiver) =
            <<Self::Factory as AppFactory>::CommandChannel as AppCommandChannel>::create();
        command_sender.into()
    }
}
