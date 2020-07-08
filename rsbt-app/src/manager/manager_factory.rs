use crate::App;

pub trait ManagerFactory<A: App> {
    type AnnounceManager;
}
