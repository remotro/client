use std::{
    net::TcpStream,
    io::{Read,Write},
    thread,
    sync::Arc
};
use crossbeam_channel::{
    unbounded,select,
};
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use tokio::{
    sync::{oneshot,Mutex},
    net::TcpStream as tokioTcpStream,
    time::{Duration,timeout}
};
pub fn handle_client(mut stream: TcpStream) {
    let (s,r) = unbounded();
    let keep_alive_stream = stream.try_clone();
    let keep_alive = thread::spawn({
        let s = s.clone();
        let r = r.clone();
        move|| {
        let max_retries = 5;
        let mut current_retries = 0;
        let max_wait = Duration::from_secs(5);
        let send = keep_alive_stream.expect("Stream does not exist").write_all(b"keepAlive\n");
        loop {
            let _ = send;
            select! {
                recv(r) -> msg => {
                    match msg {
                        Ok(..) => {
                            continue;
                        },
                        Err(_) => break,
                    }
                }
                default(max_wait) => {
                    if current_retries == max_retries {
                        let _ = s.send("Timeout");
                        break;
                    }
                    current_retries += 1;
                }
            }
        }
    }});
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
                if message == "keepAliveAck" {
                    let _ = s.send("keepAliveAck");
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
        if !r.is_empty() {
            drop(r);
            break "Time";
        }
    };
    keep_alive.join().unwrap();
    match result {
        "EOF" => println!("Connection closed"),
        "Error" => println!("Connection crashed"),
        "Time" => println!("Timeout"),
        _ => return,
    }
}

pub async fn new_handle (stream: tokioTcpStream) {
    let stream = Arc::new(Mutex::new(stream));
    let alive_stream = Arc::clone(&stream);
    let data_stream = Arc::clone(&stream);
    tokio::spawn(async move {
            let lock = alive_stream.lock().await;
            let max_retries = 5;
            let mut current_retries = 0;
            let max_wait = Duration::from_secs(5);
            loop {
                match lock.try_write(b"keepAlive\n") {
                    Ok(_) => (),
                    Err(_) => return,
                }
                let success = timeout(max_wait, lock.readable()).await;
            }
        });
    let lock = data_stream.lock().await;
    lock.writable().await;
    match lock.try_write(b"Connected\n") {
        Ok(_) => (),
        Err(_) => return,
    }
    println!("Connected");
    let exit = loop {
        lock.readable().await;
        let mut buf = [0; 1024];
        match lock.try_read(&mut buf) {
            Ok(0) => break "end",
            Ok(n) => {
                let message: String = String::from_utf8(buf[..n]
                    .to_vec()).expect("Invalid UTF-8");
                let message = message.trim();
                println!("{message}");
            }
            Err(_) => {
                break "err";
            }
        }
    };
    match exit {
        "end" => println!("Connection closed"),
        "err" => println!("Connection crashed"),
        "Time" => println!("Timeout"),
        _ => return,
    }
}
