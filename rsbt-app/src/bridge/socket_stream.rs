use futures::{AsyncRead, AsyncWrite};

pub trait SocketStream: Unpin + Send + AsyncRead + AsyncWrite {}
