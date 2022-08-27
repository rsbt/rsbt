use crate::{
    application::{App, AppTypeFactory},
    commands::Command,
    types::TypeFactory,
};
use futures::future::AbortHandle;

pub struct IncomingConnectionsManager<T: AppTypeFactory> {
    app_sender: <T as TypeFactory<Command<T, App<T>>>>::MpscSender,
}

impl<T: AppTypeFactory> IncomingConnectionsManager<T> {
    async fn spawn_new(app: &App<T>) -> AbortHandle {
        todo!()
    }
}
