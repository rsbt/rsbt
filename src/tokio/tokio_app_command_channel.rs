use crate::{
    commands::Command,
    tasks::AppCommandChannel,
    tokio::{TokioAppCommandReceiver, TokioAppCommandSender, TokioReceiver, TokioSender},
};
use tokio::sync::mpsc;

pub struct TokioAppCommandChannel;

impl AppCommandChannel for TokioAppCommandChannel {
    type CommandSender = TokioAppCommandSender;
    type CommandReceiver = TokioAppCommandReceiver;

    fn create() -> (Self::CommandSender, Self::CommandReceiver) {
        let (sender, receiver) = mpsc::channel::<Command>(1);
        (
            TokioAppCommandSender::from(TokioSender::from(sender)),
            TokioAppCommandReceiver::from(TokioReceiver::from(receiver)),
        )
    }
}
