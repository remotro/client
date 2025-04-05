use std::{net::TcpStream,io::{Read,Write}};
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
                let state: String = String::from_utf8(buffer[..n].to_vec()).expect("Invalid UTF-8");
                if state == "\n" {
                    continue;
                }
                if state == "EOF\n" {
                    break "EOF";
                }
                print!("{state}");
                let _ = stream.write_all(state.as_bytes());
            }
            Err(_) => {
                println!("Error reading Stream");
                break "Error";
            }
        }
    };
    match result {
        "EOF" => println!("Connection closed cleanly"),
        "Error" => println!("Connection crashed"),
        _ => return,
    }
}
