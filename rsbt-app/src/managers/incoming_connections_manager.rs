use crate::{
    application::{App, AppTypeFactory},
    commands::Command,
    types::TypeFactory,
};

pub struct IncomingConnectionsManager<T: AppTypeFactory> {
    app_sender: <T as TypeFactory<Command<T, App<T>>>>::MpscSender,
    sender: <T as TypeFactory<Command<T, Self>>>::MpscSender,
    receiver: <T as TypeFactory<Command<T, Self>>>::MpscReceiver,
}

/*
impl<T: AppTypeFactory> IncomingConnectionsManager<T> {
    async fn spawn_new() -> <T as TypeFactory<Command<T, Self>>>::MpscSender {

    }
}
*/
