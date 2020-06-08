use crate::tasks::{App, AppCommandReceiver, AppCommandSender};
pub trait AppCommandChannel<A: App> {
    fn create() -> (A::CommandSender, A::CommandReceiver);
}
