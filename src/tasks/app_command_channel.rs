use crate::tasks::{AppCommandReceiver, AppCommandSender};
pub trait AppCommandChannel {
    type CommandSender: AppCommandSender;
    type CommandReceiver: AppCommandReceiver;

    fn create() -> (Self::CommandSender, Self::CommandReceiver);
}
