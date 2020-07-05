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
    use futures::{Sink, SinkExt, Stream, StreamExt, future::join};
    use std::{
        fmt::Debug,
        marker::PhantomData,
        pin::Pin,
        task::{Context, Poll}, time::Duration,
    };

    pub trait TypeFactory<M> {
        type MpscSender: Sink<M, Error = anyhow::Error> + Clone + Debug + Unpin + Send + Sync;
        type MpscReceiver: Stream<Item = M> + Debug + Unpin + Send + Sync;

        fn mpsc_channel(buffer: usize) -> (Self::MpscSender, Self::MpscReceiver);
    }

    pub trait AppTypeFactory: TypeFactory<String> + TypeFactory<usize> {}

    pub struct App<T: AppTypeFactory> {
        type_factory: PhantomData<T>,
    }

    impl<T: AppTypeFactory> App<T> {
        pub fn new() -> Self {
            Self {
                type_factory: PhantomData,
            }
        }
        fn string_mpsc_channel(&self,
            buffer: usize,
        ) -> (
            <T as TypeFactory<String>>::MpscSender,
            <T as TypeFactory<String>>::MpscReceiver,
        ) {
            <T as TypeFactory<String>>::mpsc_channel(buffer)
        }

        async fn run() {
            let (sender, mut receiver) = <T as TypeFactory<usize>>::mpsc_channel(1);

            while let Some(msg) = receiver.next().await {
                Self::call_usize(msg);
            }
        }

        fn call_usize(msg: usize) {

        }
    }

    pub struct TokioTypeFactory;

    impl AppTypeFactory for TokioTypeFactory {}

    pub struct TokioMpscSender<M>(tokio::sync::mpsc::Sender<M>);

    impl<M> Clone for TokioMpscSender<M> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<M> Debug for TokioMpscSender<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TokioMpscSender({:?})", self.0)
        }
    }

    impl<M: 'static + Debug + Send + Sync> Sink<M> for TokioMpscSender<M> {
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

        fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
    }

    impl<M: 'static + Debug + Send + Sync> TypeFactory<M> for TokioTypeFactory {
        type MpscSender = TokioMpscSender<M>;
        type MpscReceiver = tokio::sync::mpsc::Receiver<M>;
        fn mpsc_channel(buffer: usize) -> (Self::MpscSender, Self::MpscReceiver) {
            let (sender, receiver) = tokio::sync::mpsc::channel(buffer);
            (TokioMpscSender(sender), receiver)
        }
    }

    pub async fn main() -> RsbtResult<()> {
        let app: App<TokioTypeFactory> = App::new();
        let (mut sender, mut receiver) = app.string_mpsc_channel(10);
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
        join(sender_loop, receiver_loop).await;
        Ok(())
    }
}
