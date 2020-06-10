use crate::{application::App, transport::IncomingConnection};

pub struct DefaultIncomingConnection;

impl<A: App> IncomingConnection<A> for DefaultIncomingConnection {}