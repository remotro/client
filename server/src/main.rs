use std::{
    net::TcpListener,
    thread::{self,JoinHandle},
};
use websocket::sync::Server;
mod sockets;

fn main() -> std::io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:34143")?;
    let _web_listener = Server::bind("127.0.0.1:34144")?;
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for stream in tcp_listener.incoming() {
        let handle = thread::spawn(move || {
            sockets::handle_client(stream.unwrap());
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}
