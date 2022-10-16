use std::future::{Future, IntoFuture};

use crate::{JoinHandle, Runtime, RuntimeJoinHandle};

pub struct TokioRuntime {
    rt: tokio::runtime::Runtime,
}

impl TokioRuntime {
    /// Creates a new runtime instance with default configuration values.
    ///
    /// This results in the multi threaded scheduler, I/O driver, and time driver being initialized.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsbt_rt::TokioRuntime;
    ///
    /// let rt = TokioRuntime::new().unwrap();
    /// ```
    pub fn new() -> std::io::Result<Self> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map(|rt| Self { rt })
    }
}

impl Runtime for TokioRuntime {
    type JoinHandle = TokioJoinHandle;
    /// Runs a future to completion on the Tokio runtime. This is the runtime’s entry point.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsbt_rt::{Runtime, TokioRuntime};
    ///
    /// let rt = TokioRuntime::new().unwrap();
    ///
    /// // Execute the future, blocking the current thread until completion
    /// rt.block_on(async {
    ///     println!("hello");
    /// });
    /// ```
    fn block_on<F: std::future::Future>(&self, future: F) -> F::Output {
        self.rt.block_on(future)
    }

    fn spawn<F>(&self, future: F) -> JoinHandle<Self>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        JoinHandle::new(TokioJoinHandle(self.rt.spawn(future)))
    }
}

pub struct TokioJoinHandle(tokio::task::JoinHandle<()>);

impl RuntimeJoinHandle for TokioJoinHandle {
    type Output = ();
}

impl IntoFuture for TokioJoinHandle {
    type Output = Result<(), tokio::task::JoinError>;

    type IntoFuture = tokio::task::JoinHandle<()>;

    fn into_future(self) -> Self::IntoFuture {
        self.0
    }
}
