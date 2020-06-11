use crate::{application::App, bridge::Receiver, commands::Command};

pub trait AppCommandReceiver<A: App>: Receiver<Command<A>> {}
