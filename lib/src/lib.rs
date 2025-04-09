use tokio::net::TcpListener;
mod tcp;
use tcp::{ManagedTcpListener, ManagedTcpStream};

pub struct Remotro {
    managed_tcp_listener: ManagedTcpListener,
}

impl Remotro {
    pub async fn host(host: impl AsRef<str>, port: u16) -> Result<Self, std::io::Error> {
        let managed_tcp_listener = ManagedTcpListener::bind(host, port).await?;
        Ok(Self { managed_tcp_listener })
    }

    pub async fn accept(&self) -> Result<Balatro, std::io::Error> {
        let managed_tcp_stream = self.managed_tcp_listener.accept().await?;
        Ok(Balatro { managed_tcp_stream })
    }
}

pub struct Balatro {
    managed_tcp_stream: ManagedTcpStream,
}
