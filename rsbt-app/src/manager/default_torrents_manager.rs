use crate::{application::App, manager::TorrentsManager};
use std::fmt::Debug;

#[derive(Debug)]
pub struct DefaultTorrentsManager;

impl<A: App> TorrentsManager<A> for DefaultTorrentsManager {}
