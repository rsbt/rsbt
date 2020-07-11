use super::{AnyRequest, AnyResult};
use crate::{
    application::AppTypeFactory,
    types::{OneshotSender, TypeFactory},
};

pub enum Command<A: AppTypeFactory, B> {
    Request(
        <A as TypeFactory<AnyResult>>::OneshotSender,
        Box<dyn AnyRequest<B> + Send + Sync>,
    ),
}

impl<A: AppTypeFactory, B> Command<A, B> {
    pub(crate) async fn execute(self, target: &mut B) {
        match self {
            Command::Request(sender, mut any_request) => {
                let any_result = any_request.any_request(target).await;
                sender.send(any_result).ok();
            }
        }
    }
}
