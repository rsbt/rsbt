use futures::{
    future::{join, BoxFuture, FutureExt},
    Future, StreamExt,
};

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
