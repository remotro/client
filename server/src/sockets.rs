use std::{
    net::TcpStream,
    io::{Read,Write}
};
pub fn handle_client(mut stream: TcpStream) {
    match stream.write_all(b"Connection success\n") {
        Ok(s) => s,
        Err(_) => return,
    }
    println!("Connection Success");
    let result = loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(0) => break "EOF",
            Ok(n) => {
                let message: String = String::from_utf8(buffer[..n].to_vec()).expect("Invalid UTF-8");
                let message = message.trim();
                if message == "action:keepAlive" {
                    let _ = stream.write_all(b"keepAliveAck\n");
                    continue;
                }
                if message == "" {
                    continue;
                }
                println!("{message}");
            }
            Err(_) => {
                println!("Error reading Stream");
                break "Error";
            }
        }
    };
    match result {
        "EOF" => println!("Connection closed"),
        "Error" => println!("Connection crashed"),
        _ => return,
    }
}
