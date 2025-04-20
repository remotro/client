pub mod balatro;
mod net;

use balatro::Balatro;
use net::Socket;

pub struct Remotro {
    socket: Socket,
}

impl Remotro {
    pub async fn host(host: impl AsRef<str>, port: u16) -> Result<Self, net::Error> {
        Ok(Self { socket: Socket::bind(host, port).await? })
    }

    pub async fn accept(&mut self) -> Result<Balatro, net::Error> {
        let connection = self.socket.accept().await?;
        Ok(Balatro::new(connection))
    }
}
