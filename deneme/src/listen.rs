use futures::prelude::*;
use std::{fmt, ops::Bound, pin::Pin};
use tokio::{io, net::TcpStream};
use tokio_stream::{Stream, wrappers::TcpListenerStream};
pub use std::io::*;
use std::net::SocketAddr;
pub use tokio::io::{
    duplex, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, DuplexStream, ReadBuf,
};


#[async_trait::async_trait]
pub trait Peek {
    /// Receives data on the socket from the remote address to which it is
    /// connected, without removing that data from the queue. On success,
    /// returns the number of bytes peeked. A return value of zero bytes does not
    /// necessarily indicate that the underlying socket has closed.
    ///
    /// Successive calls return the same data. This is accomplished by passing
    /// `MSG_PEEK` as a flag to the underlying recv system call.
    async fn peek(&self, buf: &mut [u8]) -> Result<usize>;
}

// Special-case a wrapper for TcpStream::peek.
#[async_trait::async_trait]
impl Peek for tokio::net::TcpStream {
    async fn peek(&self, buf: &mut [u8]) -> Result<usize> {
        tokio::net::TcpStream::peek(self, buf).await
    }
}

#[async_trait::async_trait]
impl Peek for tokio::io::DuplexStream {
    async fn peek(&self, _: &mut [u8]) -> Result<usize> {
        Ok(0)
    }
}


// === PeerAddr ===

pub trait PeerAddr {
    fn peer_addr(&self) -> Result<SocketAddr>;
}

impl PeerAddr for tokio::net::TcpStream {
    fn peer_addr(&self) -> Result<SocketAddr> {
        tokio::net::TcpStream::peer_addr(self)
    }
}

impl PeerAddr for tokio::io::DuplexStream {
    fn peer_addr(&self) -> Result<SocketAddr> {
        Ok(([0, 0, 0, 0], 0).into())
    }
}


pub trait Bind<T> {
    type Io: io::AsyncRead
        + io::AsyncWrite
        + Peek
        + PeerAddr
        + fmt::Debug
        + Unpin
        + Send
        + Sync
        + 'static;
    type Addrs: Clone + Send + Sync + 'static;
    type Incoming: Stream<Item = io::Result<(Self::Addrs, Self::Io)>> + Send + Sync + 'static;

    fn bind(self, params: &T) -> io::Result<Bound<Self::Incoming>>;
}


pub type Bound<I> = (Local<ServerAddr>, I);

#[derive(Copy, Clone, Debug, Default)]
pub struct BindTcp(());

#[derive(Clone, Debug)]
pub struct Addrs {
    pub server: Local<ServerAddr>,
    pub client: Remote<ClientAddr>,
}

// === impl BindTcp ===

impl BindTcp {
    pub fn with_orig_dst() -> super::BindWithOrigDst<Self> {
        super::BindWithOrigDst::from(Self::default())
    }
}

impl<T> Bind<T> for BindTcp
where
    T: Param<ListenAddr> + Param<Keepalive>,
{
    type Addrs = Addrs;
    type Incoming = Pin<Box<dyn Stream<Item = io::Result<(Self::Addrs, Self::Io)>> + Send + Sync>>;
    type Io = TcpStream;

    fn bind(self, params: &T) -> io::Result<Bound<Self::Incoming>> {
        let listen = {
            let ListenAddr(addr) = params.param();
            let l = std::net::TcpListener::bind(addr)?;
            // Ensure that O_NONBLOCK is set on the socket before using it with Tokio.
            l.set_nonblocking(true)?;
            tokio::net::TcpListener::from_std(l).expect("listener must be valid")
        };
        let server = Local(ServerAddr(listen.local_addr()?));
        let Keepalive(keepalive) = params.param();
        let accept = TcpListenerStream::new(listen).map(move |res| {
            let tcp = res?;
            super::set_nodelay_or_warn(&tcp);
            let tcp = super::set_keepalive_or_warn(tcp, keepalive)?;
            let client = Remote(ClientAddr(tcp.peer_addr()?));
            Ok((Addrs { server, client }, tcp))
        });

        Ok((server, Box::pin(accept)))
    }
}
