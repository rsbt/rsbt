use crate::{
    methods::{AnyRequest, Method},
    tasks::App,
};
use async_trait::async_trait;
use std::any::Any;

#[derive(Debug)]
pub enum Command<A> {
    Request(Box<dyn AnyRequest<A>>),
    Complex(Box<dyn Any + Send + Sync>),
}

#[async_trait]
impl<T> Method<T> for Command<T>
where
    T: App,
{
    async fn exec(&mut self, o: &mut T) {
        match self {
            Command::Request(_) => {}
            Command::Complex(_) => {}
        }
        let properties = o.properties();
        eprintln!("hello world: {:?}", properties);
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
