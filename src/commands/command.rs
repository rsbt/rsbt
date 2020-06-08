use crate::{commands::CommandRequest, methods::Method, tasks::App};
use async_trait::async_trait;
use std::any::Any;

#[derive(Debug)]
pub enum Command<A> {
    Request(Box<dyn CommandRequest<A> + Sync>),
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
/*
#[cfg(test)]
mod tests {
    use super::Command;
    use std::mem::size_of;

    #[test]
    fn size_checks() {
        assert_eq!(size_of::<Command>(), 0);
    }
}
*/
