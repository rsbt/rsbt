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
