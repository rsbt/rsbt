use crate::RsbtResult;
use std::{fmt::Debug, future::Future};

pub trait OneshotReceiver<M>: Future<Output = RsbtResult<M>> + Debug {}
