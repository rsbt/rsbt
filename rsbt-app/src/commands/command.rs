use crate::{
    application::App,
    bridge::{OneshotChannel, OneshotSender},
    methods::{AnyRequest, AnyResult, Method},
};
use async_trait::async_trait;

#[derive(Debug)]
pub enum Command<A: App> {
    Request(
        <A::AnyResultOneshotChannel as OneshotChannel<A, AnyResult>>::OneshotSender,
        Box<dyn AnyRequest<A> + Send>,
    ),
}

#[async_trait]
impl<T> Method<T> for Command<T>
where
    T: App,
{
    async fn exec(self, o: &mut T) {
        match self {
            Command::Request(sender, mut any_request) => {
                let any_result = any_request.any_request(o).await;
                sender.send(any_result).ok();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    use crate::{tests_common::size_check, TokioApp};

    #[test]
    fn size_checks() {
        size_check::<Command<TokioApp>>(3);
    }
}
