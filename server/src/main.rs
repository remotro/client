use std::{
    // Removed unused thread import
};
use tokio::net::TcpListener;
// use websocket::sync::Server; // Removed unused import
mod sockets;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:34143").await?;
    // let _web_listener = Server::bind("127.0.0.1:34144"); // Removed unused websocket listener
    // Removed unused handles vector
    println!("TCP Server listening on 127.0.0.1:34143"); // Added log
    loop {
        let (socket, addr) = tcp_listener.accept().await?;
        println!("Accepted connection from: {}", addr); // Log new connection
        // Spawn a new asynchronous task for each connection
        tokio::spawn(async move {
            sockets::new_handle(socket).await;
        });
    }
    // Removed unreachable join loop
    Ok(())
}
