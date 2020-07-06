use crate::{
    bridge::{OneshotChannel, OneshotSender, Receiver, Sender},
    methods::{AnyRequest, AnyResult, Method},
    App, RsbtResult,
};
use async_trait::async_trait;
use futures::{
    future::{join, BoxFuture, FutureExt},
    Future, StreamExt,
};
use std::any::Any;

pub enum Action<A: App, B> {
    Request(
        <A::AnyResultOneshotChannel as OneshotChannel<A, AnyResult>>::OneshotSender,
        Box<dyn AnyRequest<B> + Send>,
    ),
}

#[async_trait]
impl<A, B> Method<B> for Action<A, B>
where
    A: App,
    B: Send,
{
    async fn exec(self, o: &mut B) {
        match self {
            Action::Request(sender, mut any_request) => {
                let any_result = any_request.any_request(o).await;
                sender.send(any_result).ok();
            }
        }
    }
}

#[async_trait]
pub trait ActionHandle<A: App, T: 'static + Send> {
    type Sender: Sender<Action<A, T>, RsbtResult<()>>;
    fn sender(&mut self) -> &mut Self::Sender;

    async fn send(&mut self, action: Action<A, T>) -> RsbtResult<()> {
        self.sender().send(action).await
    }

    async fn request<'a, C: 'static, R>(&'a mut self, action_request: C) -> RsbtResult<R>
    where
        C: AnyRequest<T>,
        R: 'static,
    {
        let (sender, receiver) = A::AnyResultOneshotChannel::create();

        self.send(Action::Request(sender, Box::new(action_request)))
            .await?;

        let result = receiver.await?;

        if let Ok(any) = <Box<dyn Any + Send>>::downcast::<R>(result) {
            Ok(*any)
        } else {
            Err(anyhow::anyhow!(
                "cannot downcast from request, caller and cally types do not match"
            ))
        }
    }
}

#[async_trait]
pub trait ActionLoop<A: App>: Sized {
    type Receiver: Receiver<Action<A, Self>>;

    fn action_receiver(&mut self) -> Option<Self::Receiver>;
    async fn action_loop(mut self) {
        if let Some(mut action_receiver) = self.action_receiver() {
            while let Some(action) = action_receiver.next().await {
                action.exec(&mut self).await;
                if !self.is_running() {
                    break;
                }
            }
        }
    }
    fn is_running(&self) -> bool;
}

/*
struct App {
    receiver: tokio::sync::mpsc::Receiver<Command>,
    state: usize,
}

impl App {
    async fn change_me(&mut self, params: &[u8]) -> Vec<u8> {
        self.state += 1;
        vec![]
    }

    async fn run(&mut self) {
        while let Some(command) = self.receiver.next().await {
            command.0(self).await;
        }
    }
}

struct Command(Box<dyn FnOnce(&mut App) -> BoxFuture<'_, Vec<u8>> + Send>);

async fn do_it() {
    let data = vec![1, 2, 3];
    let command = Command(Box::new(|x| {
        async move { x.change_me(&data).await }.boxed()
    }));

    let (mut sender, receiver) = tokio::sync::mpsc::channel(1);
    let f1 = async move {
        sender.send(command).await.ok();
    };
    let mut app = App { receiver, state: 0 };
    let f2 = async move {
        app.run().await;
    };

    join(f1, f2).await;
}
*/

pub mod deep_experiments {

    use crate::RsbtResult;
    use async_trait::async_trait;
    use futures::{
        future::{join, BoxFuture},
        FutureExt, Sink, SinkExt, Stream, StreamExt,
    };
    use std::{
        any::Any,
        fmt::{Debug, Formatter},
        future::Future,
        marker::PhantomData,
        pin::Pin,
        task::{Context, Poll},
        time::Duration,
    };

    pub struct App<T: AppTypeFactory> {
        sender: <T as TypeFactory<Command<T, Self>>>::MpscSender,
        receiver: <T as TypeFactory<Command<T, Self>>>::MpscReceiver,
        type_factory: PhantomData<T>,
    }

    impl<T: AppTypeFactory> App<T> {
        pub fn new() -> Self {
            let (sender, receiver) = <T as TypeFactory<Command<T, Self>>>::mpsc_channel(10);
            Self {
                sender,
                receiver,
                type_factory: PhantomData,
            }
        }

        async fn run(mut self) {
            let mut sender = self.sender.clone();

            let incoming_connections_loop = async move {
                sender.request(|x| async move {}).await;
            };

            let command_loop = async move {
                while let Some(cmd) = self.receiver.next().await {
                    match cmd {
                        Command::Request(sender, mut any_request) => {
                            let any_result = any_request.any_request(&mut self).await;
                            sender.send(any_result).ok();
                        }
                    }
                }
            };

            join(incoming_connections_loop, command_loop).await;
        }

        async fn say_hello(&mut self) {
            eprintln!("hello new beautiful world!");
        }
    }

    pub trait TypeFactory<M> {
        type MpscSender: Sink<M, Error = anyhow::Error> + Clone + Debug + Unpin + Send + Sync;
        type MpscReceiver: Stream<Item = M> + Debug + Unpin + Send + Sync;
        type OneshotSender: OneshotSender<M> + Debug + Unpin + Send + Sync;
        type OneshotReceiver: Future<Output = RsbtResult<M>> + Debug + Unpin + Send + Sync;

        fn mpsc_channel(buffer: usize) -> (Self::MpscSender, Self::MpscReceiver);
        fn oneshot_channel() -> (Self::OneshotSender, Self::OneshotReceiver);
    }

    pub struct Handler<A: AppTypeFactory>(PhantomData<A>);

    impl<A: AppTypeFactory> Debug for Handler<A> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Handler")
        }
    }

    pub trait AppTypeFactory:
        Sized
        + TypeFactory<String>
        + TypeFactory<AnyResult>
        + TypeFactory<Command<Self, App<Self>>>
        + Sync
        + Send
        + 'static
    {
        type AppRuntime: AppRuntime;
    }

    pub trait OneshotSender<M> {
        fn send(self, message: M) -> Result<(), M>;
    }

    pub struct TokioTypeFactory;

    impl AppTypeFactory for TokioTypeFactory {
        type AppRuntime = TokioAppRuntime;
    }

    pub struct TokioMpscSender<M>(tokio::sync::mpsc::Sender<M>);

    impl<M> Clone for TokioMpscSender<M> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<M> Debug for TokioMpscSender<M> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "TokioMpscSender({:?})", self.0)
        }
    }

    impl<M> Sink<M> for TokioMpscSender<M>
    where
        M: 'static + Debug + Send + Sync,
    {
        type Error = anyhow::Error;

        fn poll_ready(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            self.0.poll_ready(cx).map_err(anyhow::Error::from)
        }

        fn start_send(mut self: Pin<&mut Self>, item: M) -> Result<(), Self::Error> {
            self.0.try_send(item).map_err(anyhow::Error::from)
        }

        fn poll_flush(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            match self.0.poll_ready(cx) {
                Poll::Ready(Err(_)) => Poll::Ready(Ok(())),
                x => x.map_err(anyhow::Error::from),
            }
        }

        fn poll_close(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
    }

    pub struct TokioOneshotSender<M>(tokio::sync::oneshot::Sender<M>);

    impl<M> Debug for TokioOneshotSender<M>
    where
        M: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "TokioOneshotSender({:?})", self.0)
        }
    }

    impl<M> OneshotSender<M> for TokioOneshotSender<M> {
        fn send(self, message: M) -> Result<(), M> {
            self.0.send(message)
        }
    }

    pub struct TokioOneshotReceiver<M>(tokio::sync::oneshot::Receiver<M>);

    impl<M: Debug> Debug for TokioOneshotReceiver<M> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "TokioOneshotReceiver({:?})", self.0)
        }
    }

    impl<M> Future for TokioOneshotReceiver<M> {
        type Output = RsbtResult<M>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            Pin::new(&mut self.0).poll(cx).map_err(anyhow::Error::from)
        }
    }

    impl<M> TypeFactory<M> for TokioTypeFactory
    where
        M: 'static + Debug + Send + Sync,
    {
        type MpscSender = TokioMpscSender<M>;
        type MpscReceiver = tokio::sync::mpsc::Receiver<M>;
        type OneshotSender = TokioOneshotSender<M>;
        type OneshotReceiver = TokioOneshotReceiver<M>;

        fn mpsc_channel(buffer: usize) -> (Self::MpscSender, Self::MpscReceiver) {
            let (sender, receiver) = tokio::sync::mpsc::channel(buffer);

            (TokioMpscSender(sender), receiver)
        }

        fn oneshot_channel() -> (Self::OneshotSender, Self::OneshotReceiver) {
            let (sender, receiver) = tokio::sync::oneshot::channel();

            (TokioOneshotSender(sender), TokioOneshotReceiver(receiver))
        }
    }

    pub trait AppRuntime {
        fn spawn<'a, F>(f: F) -> BoxFuture<'a, RsbtResult<F::Output>>
        where
            F: Future + Send + 'static,
            F::Output: Send + 'static;

        fn delay_for<'a>(duration: Duration) -> BoxFuture<'a, ()>;

        fn timeout<'a, T>(duration: Duration, future: T) -> BoxFuture<'a, RsbtResult<T::Output>>
        where
            T: Future + Send + 'a;
    }

    pub struct TokioAppRuntime;

    impl AppRuntime for TokioAppRuntime {
        fn spawn<'a, F>(f: F) -> BoxFuture<'a, RsbtResult<F::Output>>
        where
            F: Future + Send + 'static,
            F::Output: Send + 'static,
        {
            tokio::spawn(f)
                .map(|x| x.map_err(anyhow::Error::from))
                .boxed()
        }

        fn delay_for<'a>(duration: Duration) -> BoxFuture<'a, ()> {
            tokio::time::delay_for(duration).boxed()
        }

        fn timeout<'a, T>(duration: Duration, future: T) -> BoxFuture<'a, RsbtResult<T::Output>>
        where
            T: Future + Send + 'a,
        {
            tokio::time::timeout(duration, future)
                .map(|x| x.map_err(anyhow::Error::from))
                .boxed()
        }
    }

    pub async fn main() -> RsbtResult<()> {
        let app: App<TokioTypeFactory> = App::new();
        /*        let (mut sender, mut receiver) = app.mpsc_channel(10);
        let sender_loop = async move {
            eprintln!("sending message...");
            if let Err(err) = sender.send("hello, world!".into()).await {
                eprintln!("{}", err);
            }
            eprintln!("wait 10 seconds...");
            tokio::time::delay_for(Duration::from_secs(10)).await;
            eprintln!("done sender");
        };
        let receiver_loop = async move {
            while let Some(message) = receiver.next().await {
                eprintln!("{}", message);
            }
            eprintln!("done receiver");
        };
        join(sender_loop, receiver_loop).await;*/
        Ok(())
    }

    pub enum Command<A: AppTypeFactory, B> {
        Request(
            <A as TypeFactory<AnyResult>>::OneshotSender,
            Box<dyn AnyRequest<B> + Send + Sync>,
        ),
    }

    impl<A: AppTypeFactory, B> Debug for Command<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Command")
        }
    }

    pub(crate) type AnyResult = Box<dyn Any + Send + Sync>;

    #[async_trait]
    pub trait AnyRequest<A> {
        async fn any_request(&mut self, o: &mut A) -> AnyResult;
    }

    fn box_any<R>(any: R) -> AnyResult
    where
        R: Send + Sync + 'static,
    {
        Box::new(any)
    }

    struct AnyCommand<'a, A: 'a, R>(
        Option<Box<dyn FnOnce(&mut A) -> BoxFuture<'a, R> + Send + Sync>>,
    );

    #[async_trait]
    impl<A: Send, R: Send + Sync + 'static> AnyRequest<A> for AnyCommand<'_, A, R> {
        async fn any_request(&mut self, o: &mut A) -> AnyResult {
            if let Some(command) = self.0.take() {
                let result: AnyResult = Box::new(command(o).await);
                result
            } else {
                panic!();
            }
        }
    }

    trait CommandSender<A, B>:
        Sink<Command<A, B>, Error = anyhow::Error> + Clone + Debug + Unpin + Send + Sync
    where
        A: AppTypeFactory,
        B: Sync + Send + 'static,
    {
        fn request<F, R>(&mut self, f: F) -> BoxFuture<'_, RsbtResult<R::Output>>
        where
            F: FnOnce(&mut B) -> R + Send + Sync + 'static,
            R: Future + Send + Sync + 'static,
            R::Output: Send + Sync + 'static,
        {
            Box::pin(async move {
                let (sender, receiver) = <A as TypeFactory<AnyResult>>::oneshot_channel();

                self.send(Command::Request(
                    sender,
                    Box::new(AnyCommand(Some(Box::new(|x| f(x).boxed())))),
                ))
                .await?;

                let result = receiver.await?;

                if let Ok(any) = <Box<dyn Any + Send>>::downcast::<R::Output>(result) {
                    Ok(*any)
                } else {
                    Err(anyhow::anyhow!(
                        "cannot downcast from request, caller and cally types do not match"
                    ))
                }
            })
        }
    }

    impl<T, A: AppTypeFactory, B> CommandSender<A, B> for T
    where
        B: Sync + Send + 'static,
        T: Sink<Command<A, B>, Error = anyhow::Error> + Clone + Debug + Unpin + Send + Sync,
    {
    }
    /*

    impl<T, A: AppTypeFactory> CommandSender<A, App<A>> for T where
        T: Sink<Command<A, App<A>>, Error = anyhow::Error> + Clone + Debug + Unpin + Send + Sync
    {
    }
    */
}
