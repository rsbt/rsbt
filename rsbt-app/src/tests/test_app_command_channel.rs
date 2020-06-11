use crate::{
    application::AppCommandChannel,
    tests::{TestApp, TestAppCommandReceiver, TestAppCommandSender},
};

pub struct TestAppCommandChannel {}

impl AppCommandChannel<TestApp> for TestAppCommandChannel {
    fn create() -> (TestAppCommandSender, TestAppCommandReceiver) {
        (TestAppCommandSender, TestAppCommandReceiver)
    }
}
