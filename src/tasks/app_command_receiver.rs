use crate::{commands::Command, tasks::Receiver};

pub trait AppCommandReceiver: Receiver<Command> {}
