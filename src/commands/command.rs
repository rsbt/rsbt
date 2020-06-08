use crate::{methods::Method, tasks::App};
use std::any::Any;
use async_trait::async_trait;


#[derive(Debug)]
pub enum Command {
    Simple,
    Complex(Box<dyn Any + Send + Sync>),
}

#[async_trait]
impl<T> Method<T> for Command
where
    T: App,
{
    async fn exec(&mut self, o: &mut T) {
        match self {
            Command::Simple => {}
            Command::Complex(_) => {}
        }
        let properties = o.properties();
        eprintln!("hello world: {:?}", properties);
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    use std::mem::size_of;

    #[test]
    fn size_checks() {
        assert_eq!(size_of::<Command>(), 0);
    }
}
