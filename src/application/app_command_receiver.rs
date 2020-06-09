use crate::{
    commands::Command,
    application::{App, Receiver},
};

pub trait AppCommandReceiver<A: App>: Receiver<Command<A>> {}
