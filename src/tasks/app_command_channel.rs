use crate::tasks::App;

pub trait AppCommandChannel<A: App> {
    fn create() -> (A::CommandSender, A::CommandReceiver);
}
