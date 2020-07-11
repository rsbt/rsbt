use super::AppTypeFactory;
use crate::{
    commands::{Command, CommandSender},
    torrent::{TorrentProcess, TorrentProcessStatus},
    transport::SocketListener,
    types::TypeFactory,
    RsbtResult, SHA1_SIZE,
};
use async_trait::async_trait;
use futures::{
    future::{abortable, join, BoxFuture},
    FutureExt, StreamExt,
};
use log::{debug, error, info};
use rsbt_bencode::Torrent;
use std::{
    convert::TryInto,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};

pub struct App<T: AppTypeFactory> {
    sender: <T as TypeFactory<Command<T, Self>>>::MpscSender,
    receiver: <T as TypeFactory<Command<T, Self>>>::MpscReceiver,
    properties: T::AppProperties,
    data: Vec<u8>,
}

impl<T: AppTypeFactory> App<T> {
    pub fn new(properties: T::AppProperties) -> Self {
        let (sender, receiver) = <T as TypeFactory<Command<T, Self>>>::mpsc_channel(10);
        Self {
            sender,
            receiver,
            properties,
            data: vec![],
        }
    }

    pub async fn run(mut self) {
        let mut sender = self.sender.clone();

        let data = vec![5];
        let incoming_connections_loop = async move {
            let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
            match T::SocketListener::bind(socket).await {
                Ok(mut listener) => {
                    eprintln!("listen on {}", socket);
                    while let Some(socket) = listener.next().await {}
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
            let result = request!(sender, |x: &mut Self| x.say_hello(data));
            eprintln!("{:?}", result);
        };
        /*
                let handler: Handler<T> = Handler::new(self.sender.clone());
                T::AppRuntime::spawn(async move {
                    handler.run().await;
                });
        */
        let command_loop = async move {
            while let Some(cmd) = self.receiver.next().await {
                cmd.execute(&mut self).await;
            }
        };

        join(incoming_connections_loop, command_loop).await;
    }

    async fn start(&mut self) {
        info!("starting torrent app...");
    }

    async fn say_hello(&mut self, data: Vec<u8>) -> String {
        eprintln!("hello new beautiful world! {}", data.len());
        self.data = data;
        "check me".into()
    }
}

/*
    async fn add_torrent(
        &mut self,
        data: Vec<u8>,
        filename: String,
        state: TorrentProcessStatus,
    ) -> RsbtResult<TorrentToken> {
        self.torrent_manager()
            .add_torrent(data, filename, state)
            .await
    }

    fn find_torrent_by_hash_id(&self, hash_id: &[u8; SHA1_SIZE]) -> Option<&TorrentProcess> {
        todo!()
    }
*/
