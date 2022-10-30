use crate::{Actor, ActorHandle, Input, Output};

pub struct Download<I: Input, O: Output> {
    input: I,
    output: O,
}

impl<I: Input, O: Output> Download<I, O> {
    pub fn new(input: I, output: O) -> Self {
        Self { input, output }
    }
}

impl<I: Input, O: Output> Actor for Download<I, O> {
    type Handle = DownloadHandle;
}

pub struct DownloadHandle {}

impl ActorHandle for DownloadHandle {}
