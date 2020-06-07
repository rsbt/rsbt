use crate::{tasks::AppFactory, tokio::TokioAppCommandChannel};

pub struct TokioFactory;

impl AppFactory for TokioFactory {
    type CommandChannel = TokioAppCommandChannel;
}
