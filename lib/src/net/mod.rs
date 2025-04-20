pub mod protocol;
mod tcp;

use tcp::TcpStreamExt;
use protocol::Request;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

use std::borrow::Cow;

pub struct Socket {
    listener: TcpListener,
}

impl Socket {
    pub async fn bind(host: impl AsRef<str>, port: u16) -> Result<Self, Error> {
        let listener = TcpListener::bind(format!("{}:{}", host.as_ref(), port)).await?;
        Ok(Self { listener })
    }

    pub async fn accept(&mut self) -> Result<Connection, Error> {
        let (stream, _) = self.listener.accept().await?;
        Ok(Connection::new(TcpStreamExt::new(stream)))
    }
}

pub struct Connection {
    stream: TcpStreamExt,
}

impl Connection {
    fn new(stream: TcpStreamExt) -> Self {
        Self { stream }
    }

    pub async fn req<R: Request>(&mut self, req: R) -> Result<R::Expect, Error> {
        self.stream.send(req).await?;
        self.stream.recv().await
    }
}


#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    MalformedHead(Cow<'static, str>),
    MalformedBody(serde_json::Error),
    Timeout,
    ConnectionClosed,
    Send(mpsc::error::SendError<String>)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::MalformedBody(err)
    }
}

impl From<mpsc::error::SendError<String>> for Error {
    fn from(err: mpsc::error::SendError<String>) -> Self {
        Error::Send(err)
    }
}

