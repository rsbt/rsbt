use crate::{application::App, experiments::ActionLoop, manager::TorrentsManager};
use std::fmt::Debug;

#[derive(Debug)]
pub struct DefaultTorrentsManager;

impl<A: App> TorrentsManager<A> for DefaultTorrentsManager {}
