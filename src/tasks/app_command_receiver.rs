use crate::{
    commands::Command,
    tasks::{App, Receiver},
};

pub trait AppCommandReceiver<A: App>: Receiver<Command<A>> {}
