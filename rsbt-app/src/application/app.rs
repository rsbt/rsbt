use super::{AppProperties, AppRuntime, AppTypeFactory};
use crate::{
    commands::{Command, CommandSender},
    config,
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
    time::Duration,
};

pub struct App<T: AppTypeFactory> {
    sender: <T as TypeFactory<Command<T, Self>>>::MpscSender,
    receiver: <T as TypeFactory<Command<T, Self>>>::MpscReceiver,
    properties: T::AppProperties,
    running: bool,
}

impl<T: AppTypeFactory> App<T> {
    pub fn new(properties: T::AppProperties) -> Self {
        let (sender, receiver) =
            <T as TypeFactory<Command<T, Self>>>::mpsc_channel(properties.mpsc_buffer_size());
        Self {
            sender,
            receiver,
            properties,
            running: false,
        }
    }

    pub async fn spawn(self) -> <T as TypeFactory<Command<T, Self>>>::MpscSender {
        let sender = self.sender.clone();

        <T::AppRuntime as AppRuntime>::spawn(self.run());

        sender
    }

    pub async fn run(mut self) {
        let mut sender = self.sender.clone();

        let listen_addr = self.properties.listen_addr().clone();

        let incoming_connections_loop = async move {
            match T::SocketListener::bind(listen_addr).await {
                Ok(mut listener) => {
                    info!("listen on {}", listen_addr);
                    while let Some(socket) = listener.next().await {}
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        };

        let command_loop = async move {
            while let Some(cmd) = self.receiver.next().await {
                cmd.execute(&mut self).await;
            }
        };

        join(incoming_connections_loop, command_loop).await;
    }

    pub async fn start(&mut self) {
        info!("starting torrent app...");
        self.running = true;
        info!("started torrent app");
    }

    pub async fn stop(&mut self) {
        info!("stopping torrent app...");
        self.running = false;
        info!("stopped torrent app");
    }

    pub async fn toggle(&mut self) {
        if self.running {
            self.stop().await
        } else {
            self.start().await
        }
    }

    pub async fn check_need_initial_configuration(
        custom_config_dir: Option<PathBuf>,
    ) -> RsbtResult<bool> {
        Ok(<T::AppRuntime as AppRuntime>::spawn_blocking(move || {
            config::need_initial_configuration(custom_config_dir)
        })
        .await?)
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
