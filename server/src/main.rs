use std::{
    thread::{self,JoinHandle},
};
use tokio::net::TcpListener;
use websocket::sync::Server;
mod sockets;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:34143").await?;
    let _web_listener = Server::bind("127.0.0.1:34144");
    let mut handles: Vec<JoinHandle<()>> = vec![];
    loop {
        let (socket, _) = tcp_listener.accept().await?;
        sockets::new_handle(socket).await;
    }
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}
