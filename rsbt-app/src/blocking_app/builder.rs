use rsbt_rt::Runtime;

use crate::{App, BlockingApp};

pub struct BlockingAppBuilder<R: Runtime> {
    pub(super) runtime: Option<R>,
}

impl<R> BlockingAppBuilder<R>
where
    R: Runtime,
{
    pub fn runtime(mut self, runtime: R) -> Self {
        self.runtime = Some(runtime);
        self
    }

    pub fn build(self) -> BlockingApp<R> {
        BlockingApp {
            app: App {},
            runtime: self.runtime.expect("runtime"),
        }
    }
}
