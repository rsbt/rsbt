use crate::{
    application::{App, AppTypeFactory},
    commands::Command,
    types::TypeFactory,
};
use std::fmt::{Debug, Formatter};
pub struct IncomingConnectionsManager<T: AppTypeFactory> {
    app_sender: <T as TypeFactory<Command<T, App<T>>>>::MpscSender,
    sender: <T as TypeFactory<Command<T, Self>>>::MpscSender,
    receiver: <T as TypeFactory<Command<T, Self>>>::MpscReceiver,
}

impl<A: AppTypeFactory> Debug for IncomingConnectionsManager<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IncomingConnectionsManager")
    }
}

/*
impl<T: AppTypeFactory> Debug for Command<T, IncomingConnectionsManager<T>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
*/
/*
impl<T: AppTypeFactory> IncomingConnectionsManager<T> {
    async fn spawn_new() -> <T as TypeFactory<Command<T, Self>>>::MpscSender {

    }
}
*/
