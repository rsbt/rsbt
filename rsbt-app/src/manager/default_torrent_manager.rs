use crate::{application::App, manager::TorrentManager};

#[derive(Debug)]
pub struct DefaultTorrentManager;

impl<A: App> TorrentManager<A> for DefaultTorrentManager {}
