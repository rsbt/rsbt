use crate::{
    tasks::{App, AppFactory},
    tokio::TokioFactory,
};

pub struct TokioApp;

impl App for TokioApp {
    type Factory = TokioFactory;
}
