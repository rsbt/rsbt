use crate::application::App;

pub trait AppCommandChannel<A: App> {
    fn create() -> (A::CommandSender, A::CommandReceiver);
}
