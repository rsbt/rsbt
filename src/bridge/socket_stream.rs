use futures::{AsyncRead, AsyncWrite};

pub trait SocketStream: Send + AsyncRead + AsyncWrite {}
