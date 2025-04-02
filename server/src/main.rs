use std::net::{TcpListener, TcpStream};
use websocket::sync::Server;

fn handle_client(stream: TcpStream) {

}
fn main() -> std::io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:80")?; //TODO Choose a port to run on
    let web_listener = Server::bind("127.0.0.1:0")?;

    // accept connections and process them serially
    for stream in tcp_listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
