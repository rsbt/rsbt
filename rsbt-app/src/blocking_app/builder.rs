use crate::{tokio::Runtime, App, BlockingApp};

pub struct BlockingAppBuilder {
    pub(super) runtime: Option<Runtime>,
}

impl Default for BlockingAppBuilder {
    fn default() -> Self {
        Self {
            runtime: Default::default(),
        }
    }
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
