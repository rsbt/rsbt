use crate::{Actor, ActorHandle};

pub struct BlockingActorHandle<A>(ActorHandle<A>)
where
    A: Actor;
