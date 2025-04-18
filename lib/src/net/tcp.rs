use std::borrow::Cow;
use tokio::net::TcpListener;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};
use crate::net::protocol::Request;

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
        Ok(Connection::new(stream))
    }
}

pub struct Connection {
    writer: BufWriter<OwnedWriteHalf>,
    reader: BufReader<OwnedReadHalf>,
}

impl Connection {
    fn new(stream: TcpStream) -> Self {
        let (reader, writer) = stream.into_split();
        let reader = BufReader::new(reader);
        Self {
            writer: BufWriter::new(writer),
            reader: reader,
        }
    }

    pub async fn send<R: Request>(&mut self, msg: R) -> Result<R::Expect, Error> {
        let body = serde_json::to_string(&msg)?;
        let packet = format!("{}!{}", R::kind(), body);

        self.writer.write_all(packet.as_bytes()).await?;
        self.writer.flush().await?;
        let mut buf = String::new();
        self.reader.read_line(&mut buf).await?;

        let mut split = buf.split('!');
        let kind = split
            .next()
            .ok_or(Error::Message(Cow::Borrowed("No kind")))?;
        let body = split
            .next()
            .ok_or(Error::Message(Cow::Borrowed("No body")))?;
        if let Some(_) = split.next() {
            return Err(Error::Message(Cow::Borrowed("message has more than one !")));
        }
        if kind != R::kind() {
            return Err(Error::Message(Cow::Owned(format!(
                "Expected response kind {}, got {}",
                R::kind(),
                kind
            ))));
        }

        let response: R::Expect = serde_json::from_str(body)?;
        Ok(response)
    }
}


#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Message(Cow<'static, str>),
    Json(serde_json::Error),
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
        Error::Json(err)
    }
}
