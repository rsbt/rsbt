use futures::{AsyncRead, AsyncWrite};

pub trait SocketStream: AsyncRead + AsyncWrite {}
