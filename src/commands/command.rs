use crate::{
    methods::{AnyRequest, AnyResult, Method},
    tasks::{App, OneshotChannel, OneshotSender},
};
use async_trait::async_trait;

#[derive(Debug)]
pub enum Command<A: App> {
    Request(
        <A::AnyResultOneshotChannel as OneshotChannel<A, AnyResult>>::OneshotSender,
        Box<dyn AnyRequest<A> + 'static + Send>,
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
    use crate::TokioApp;
    use std::mem::size_of;

    fn size_check<T>(required: usize) {
        assert_eq!(size_of::<T>(), size_of::<usize>() * required);
    }

    #[test]
    fn size_checks() {
        size_check::<Command<TokioApp>>(3);
    }
}
