use tokio::net::TcpListener;

pub struct Remotro {
    tcp_listener: TcpListener,
}

impl Remotro {
    pub async fn host(host: impl AsRef<str>, port: u16) -> Result<Self, std::io::Error> {
        let tcp_listener = TcpListener::bind(format!("{}:{}", host.as_ref(), port)).await?;
        Ok(Self { tcp_listener })
    }
}
