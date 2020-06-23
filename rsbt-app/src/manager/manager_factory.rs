use crate::{experiments::ActionLoop, App};

pub trait ManagerFactory<A: App> {
    type AnnounceManager: ActionLoop<A>;
}
