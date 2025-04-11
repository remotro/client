mod tcp;
use tcp::{ManagedTcpListener, ManagedTcpStream};
use std::net::SocketAddr;
use tokio::sync::mpsc::error;

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

impl Balatro {
    pub fn addr(&self) -> SocketAddr {
        self.managed_tcp_stream.addr()
    }
    pub async fn send_message(&self,message: &str) -> 
        Result<(), error::SendError<String>> {
            self.managed_tcp_stream.send_message(message).await
    }
    pub async fn recv_message(&mut self) -> Option<String> {
        self.managed_tcp_stream.recv_message().await
    }
            
}
