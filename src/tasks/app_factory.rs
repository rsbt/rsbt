use crate::tasks::AppCommandChannel;

pub trait AppFactory {
    type CommandChannel: AppCommandChannel;
}
