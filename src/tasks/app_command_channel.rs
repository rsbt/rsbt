use crate::tasks::{AppCommandReceiver, AppCommandSender};
pub trait AppCommandChannel<CommandSender: AppCommandSender, CommandReceiver: AppCommandReceiver> {
    fn create() -> (CommandSender, CommandReceiver);
}
