use crate::tokio::TokioOneshotChannel;

pub type TestOneshotChannel<M, A> = TokioOneshotChannel<M, A>;
