mod default_torrents_manager;
mod manager_factory;
mod torrents_manager;

pub(crate) use default_torrents_manager::DefaultTorrentsManager;
pub use manager_factory::ManagerFactory;
pub use torrents_manager::TorrentsManager;
