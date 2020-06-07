use crate::tasks::AppCommandSender;

pub struct AppHandler<S>(S);

impl<S: AppCommandSender> From<S> for AppHandler<S> {
    fn from(value: S) -> Self {
        Self(value)
    }
}
