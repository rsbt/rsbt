use crate::{application::App, manager::TorrentManager};

pub struct DefaultTorrentManager;

impl<A: App> TorrentManager<A> for DefaultTorrentManager {}
