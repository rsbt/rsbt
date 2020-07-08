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
        pin::Pin,
        task::{Context, Poll},
        time::Duration,
    };

    macro_rules! request {
        ($sender:ident, |$x:ident: &mut $xt:ty| $expression:expr) => {
            $sender
                .request(move |$x: &mut $xt| $expression.boxed())
                .await
        };
    }

    pub struct App<T: AppTypeFactory> {
        sender: <T as TypeFactory<Command<T, Self>>>::MpscSender,
        receiver: <T as TypeFactory<Command<T, Self>>>::MpscReceiver,
        data: Vec<u8>,
    }

    impl<T: AppTypeFactory> App<T> {
        pub fn new() -> Self {
            let (sender, receiver) = <T as TypeFactory<Command<T, Self>>>::mpsc_channel(10);
            Self {
                sender,
                receiver,
                data: vec![],
            }
        }

        async fn run(mut self) {
            let mut sender = self.sender.clone();

            let data = vec![5];
            let incoming_connections_loop = async move {
                let result = request!(sender, |x: &mut Self| x.say_hello(data));
                eprintln!("{:?}", result);
            };

            let handler: Handler<T> = Handler::new(self.sender.clone());
            T::AppRuntime::spawn(async move {
                handler.run().await;
            });

            let command_loop = async move {
                while let Some(cmd) = self.receiver.next().await {
                    cmd.execute(&mut self).await;
                }
            };

            join(incoming_connections_loop, command_loop).await;
        }

        async fn say_hello(&mut self, data: Vec<u8>) -> String {
            eprintln!("hello new beautiful world! {}", data.len());
            self.data = data;
            "check me".into()
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

    pub struct Handler<T: AppTypeFactory> {
        app_sender: <T as TypeFactory<Command<T, App<T>>>>::MpscSender,
        sender: <T as TypeFactory<Command<T, Self>>>::MpscSender,
        receiver: <T as TypeFactory<Command<T, Self>>>::MpscReceiver,
        data: Vec<u8>,
    }

    impl<A: AppTypeFactory> Debug for Handler<A> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Handler")
        }
    }

    impl<T: AppTypeFactory> Handler<T> {
        pub fn new(app_sender: <T as TypeFactory<Command<T, App<T>>>>::MpscSender) -> Self {
            let (sender, receiver) = <T as TypeFactory<Command<T, Self>>>::mpsc_channel(10);
            Self {
                app_sender,
                sender,
                receiver,
                data: vec![],
            }
        }

        async fn run(mut self) {
            let mut sender = self.sender.clone();
            let mut app_sender = self.app_sender.clone();

            let data = vec![5];
            let incoming_connections_loop = async move {
                let result = request!(sender, |x: &mut Self| x.say_hello(data));
                eprintln!("handler: {:?}", result);
                let data = result.unwrap().clone().as_bytes().to_vec();
                let result = request!(app_sender, |x: &mut App<T>| x.say_hello(data));
                eprintln!("app: {:?}", result);
            };

            let command_loop = async move {
                while let Some(cmd) = self.receiver.next().await {
                    cmd.execute(&mut self).await;
                }
            };

            join(incoming_connections_loop, command_loop).await;
        }

        async fn say_hello(&mut self, data: Vec<u8>) -> String {
            eprintln!("hello new beautiful world from handle!");
            self.data = data;
            "check me too from handle".into()
        }
    }

    pub trait AppTypeFactory:
        Sized
        + TypeFactory<String>
        + TypeFactory<AnyResult>
        + TypeFactory<Command<Self, App<Self>>>
        + TypeFactory<Command<Self, Handler<Self>>>
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
        M: Debug + Send + Sync,
    {
        type Error = anyhow::Error;

        fn poll_ready(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            self.0.poll_ready(cx).map_err(anyhow::Error::from)
        }

        fn start_send(mut self: Pin<&mut Self>, item: M) -> Result<(), Self::Error> {
            self.0.try_send(item).map_err(|e| anyhow::anyhow!("{}", e))
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
        M: Debug + Send + Sync,
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
        let _ = app.run().await;
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

    impl<A: AppTypeFactory, B> Command<A, B> {
        async fn execute(self, target: &mut B) {
            match self {
                Command::Request(sender, mut any_request) => {
                    let any_result = any_request.any_request(target).await;
                    sender.send(any_result).ok();
                }
            }
        }
    }

    pub(crate) type AnyResult = Box<dyn Any + Send + Sync>;

    #[async_trait]
    pub trait AnyRequest<A> {
        async fn any_request(&mut self, o: &mut A) -> AnyResult;
    }

    struct AnyCommand<A, R: 'static>(
        Option<Box<dyn FnOnce(&mut A) -> BoxFuture<'_, R> + Send + Sync>>,
    );

    #[async_trait]
    impl<A, R> AnyRequest<A> for AnyCommand<A, R>
    where
        A: Send,
        R: Send + Sync + 'static,
    {
        async fn any_request(&mut self, o: &mut A) -> AnyResult {
            if let Some(command) = self.0.take() {
                let result: AnyResult = Box::new(command(o).await);
                result
            } else {
                panic!();
            }
        }
    }

    #[async_trait]
    trait CommandSender<A, B>:
        Sink<Command<A, B>, Error = anyhow::Error> + Clone + Debug + Unpin + Send + Sync
    where
        A: AppTypeFactory,
        B: Sync + Send + 'static,
    {
        async fn request<F, R>(&mut self, f: F) -> RsbtResult<R>
        where
            F: FnOnce(&mut B) -> BoxFuture<'_, R> + Send + Sync + 'static,
            R: Send + Sync + 'static,
        {
            let (sender, receiver) = <A as TypeFactory<AnyResult>>::oneshot_channel();

            self.send(Command::Request(
                sender,
                Box::new(AnyCommand(Some(Box::new(f)))),
            ))
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

    impl<T, A: AppTypeFactory, B> CommandSender<A, B> for T
    where
        B: Sync + Send + 'static,
        T: Sink<Command<A, B>, Error = anyhow::Error> + Clone + Debug + Unpin + Send + Sync,
    {
    }
}
