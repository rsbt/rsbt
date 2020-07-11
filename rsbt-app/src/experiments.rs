pub mod deep_experiments {

    use crate::{
        application::{App, AppRuntime, AppTypeFactory},
        commands::{AnyCommand, AnyRequest, AnyResult, Command},
        tokio::TokioTypeFactory,
        transport::SocketConnect,
        types::{OneshotSender, TypeFactory},
        RsbtAppProperties, RsbtResult,
    };
    use async_trait::async_trait;
    use futures::{
        future::{join, BoxFuture},
        stream::BoxStream,
        AsyncRead, AsyncWrite, FutureExt, Sink, SinkExt, Stream, StreamExt,
    };
    use std::{
        any::Any,
        fmt::{Debug, Formatter},
        future::Future,
        net::{IpAddr, Ipv4Addr, SocketAddr},
        pin::Pin,
        task::{Context, Poll},
        time::Duration,
    };
    use tokio::net::{TcpListener, TcpStream};
    use tokio_util::compat::{Compat, Tokio02AsyncReadCompatExt};

    pub async fn main() -> RsbtResult<()> {
        let app: App<TokioTypeFactory> = App::new(Default::default());
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
}
