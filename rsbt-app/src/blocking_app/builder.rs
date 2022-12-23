use crate::{tokio::Runtime, App, BlockingApp};

#[derive(Default)]
pub struct BlockingAppBuilder {
    pub(super) runtime: Option<Runtime>,
}

impl BlockingAppBuilder {
    pub fn runtime(mut self, runtime: Runtime) -> Self {
        self.runtime = Some(runtime);
        self
    }

    pub fn build(self) -> BlockingApp {
        BlockingApp {
            app: App {},
            runtime: self.runtime.expect("runtime"),
        }
    }
}
