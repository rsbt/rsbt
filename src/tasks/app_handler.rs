use crate::tasks::AppCommandSender;

#[derive(Clone)]
pub struct AppHandler<S>(S);

impl<S: AppCommandSender> From<S> for AppHandler<S> {
    fn from(value: S) -> Self {
        Self(value)
    }
}

impl<S: AppCommandSender> AppHandler<S> {}
