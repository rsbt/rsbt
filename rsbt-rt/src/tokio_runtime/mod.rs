use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use futures::TryFutureExt;

use crate::{JoinError, Runtime, RuntimeHandle};

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
    type Handle = TokioHandle;

    fn handle(&self) -> Self::Handle {
        TokioHandle(self.rt.handle().clone())
    }

    fn shutdown_background(self) {
        self.rt.shutdown_background()
    }
}

impl RuntimeHandle for TokioRuntime {
    type JoinHandle<O> = TokioJoinHandle<O> where O: Send + 'static;
    type MpscSender<T> = TokioMpscSender<T>where
    T: Send + 'static;
    type MpscReceiver<T> = TokioMpscReceiver<T>where
    T: Send + Unpin + 'static;
    /// Runs a future to completion on the Tokio runtime. This is the runtime’s entry point.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsbt_rt::{Runtime, RuntimeHandle, TokioRuntime};
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

    /// Spawns a future onto the Tokio runtime.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsbt_rt::{Runtime, RuntimeHandle, TokioRuntime};
    ///
    /// // Create the runtime
    /// let rt = TokioRuntime::new().unwrap();
    ///
    /// // Spawn a future onto the runtime
    /// rt.spawn(async {
    ///     println!("now running on a worker thread");
    /// });
    /// ```
    fn spawn<F>(&self, future: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        TokioJoinHandle(self.rt.spawn(future))
    }

    fn spawn_current<F>(future: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        TokioJoinHandle(tokio::spawn(future))
    }

    fn channel<T: Send + Unpin + 'static>(
        buffer: usize,
    ) -> (Self::MpscSender<T>, Self::MpscReceiver<T>) {
        channel(buffer)
    }
}

pub struct TokioJoinHandle<T>(tokio::task::JoinHandle<T>);

impl<T: Send + 'static> IntoFuture for TokioJoinHandle<T> {
    type Output = Result<T, JoinError>;

    type IntoFuture = Pin<Box<dyn Future<Output = Result<T, JoinError>> + Send + 'static>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.0.map_err(JoinError::Tokio1))
    }
}

#[derive(Clone)]
pub struct TokioHandle(tokio::runtime::Handle);

impl RuntimeHandle for TokioHandle {
    type JoinHandle<O> = TokioJoinHandle<O> where O: Send + 'static;
    type MpscSender<T> = TokioMpscSender<T> where
    T: Send + 'static;
    type MpscReceiver<T> = TokioMpscReceiver<T> where
    T: Send + Unpin + 'static;
    /// Runs a future to completion on the Tokio runtime. This is the runtime’s entry point.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsbt_rt::{Runtime, RuntimeHandle, TokioRuntime};
    ///
    /// let rt = TokioRuntime::new().unwrap();
    ///
    /// let handle = rt.handle();
    ///
    /// // Execute the future, blocking the current thread until completion
    /// handle.block_on(async {
    ///     println!("hello");
    /// });
    /// ```
    fn block_on<F: std::future::Future>(&self, future: F) -> F::Output {
        self.0.block_on(future)
    }

    /// Spawns a future onto the Tokio runtime.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsbt_rt::{Runtime, RuntimeHandle, TokioRuntime};
    ///
    /// // Create the runtime
    /// let rt = TokioRuntime::new().unwrap();
    ///
    /// let handle = rt.handle();
    ///
    /// // Spawn a future onto the runtime
    /// handle.spawn(async {
    ///     println!("now running on a worker thread");
    /// });
    /// ```
    fn spawn<F>(&self, future: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        TokioJoinHandle(self.0.spawn(future))
    }

    fn spawn_current<F>(future: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        TokioJoinHandle(tokio::spawn(future))
    }

    fn channel<T: Send + Unpin + 'static>(
        buffer: usize,
    ) -> (Self::MpscSender<T>, Self::MpscReceiver<T>) {
        channel(buffer)
    }
}

#[inline]
fn channel<T: Send + 'static>(buffer: usize) -> (TokioMpscSender<T>, TokioMpscReceiver<T>) {
    let (sender, receiver) = tokio::sync::mpsc::channel(buffer);
    (
        TokioMpscSender::new(sender),
        TokioMpscReceiver::from(receiver),
    )
}

pub type TokioMpscSender<T> = tokio_util::sync::PollSender<T>;

pub type TokioMpscReceiver<T> = tokio_stream::wrappers::ReceiverStream<T>;
