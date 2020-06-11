use crate::{
    application::AppCommandChannel,
    commands::Command,
    tokio::{TokioApp, TokioAppCommandReceiver, TokioAppCommandSender, TokioReceiver, TokioSender},
};
use tokio::sync::mpsc;

pub struct TokioAppCommandChannel;

impl AppCommandChannel<TokioApp> for TokioAppCommandChannel {
    fn create() -> (TokioAppCommandSender, TokioAppCommandReceiver) {
        let (sender, receiver) = mpsc::channel::<Command<TokioApp>>(1);
        (
            TokioAppCommandSender::from(TokioSender::from(sender)),
            TokioAppCommandReceiver::from(TokioReceiver::from(receiver)),
        )
    }
}
