use std::{
    net::{TcpListener, TcpStream},
    io::{Read, Write}
};
use websocket::sync::Server;

fn handle_client(mut stream: TcpStream) {
    match stream.write_all(b"Connection success\n") {
        Ok(s) => s,
        Err(_) => return,
    }
    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let state:String = String::from_utf8(buffer[..n]
                    .to_vec()).expect("Invalid UTF-8");
                if state == "\n" { continue; }
                if state == "EOF\n" { return; }
                print!("{state}");
                let _ = stream.write_all(state.as_bytes());
            }
            Err(_) => {
                println!("Error reading Stream");
            }
        }
    }
}
fn main() -> std::io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080")?; //TODO Choose a port to run on
    /*let web_listener = Server::bind("127.0.0.1:0")?;*/

    // accept connections and process them serially
    for stream in tcp_listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
