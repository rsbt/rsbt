use crate::tasks::{App, OneshotReceiver, OneshotSender};
use std::fmt::Debug;

pub trait OneshotChannel<A: App, M>: Debug {
    type OneshotSender: OneshotSender<M>;
    type OneshotReceiver: OneshotReceiver<M>;
    fn create() -> (Self::OneshotSender, Self::OneshotReceiver);
}
