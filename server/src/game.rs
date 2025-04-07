use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    time::{interval, Duration},
};

const KEEP_ALIVE_MSG: &str = "action:keepAlive\n";
const KEEP_ALIVE_ACK_MSG: &str = "action:keepAliveAck\n";
const CONNECTED_MSG: &str = "Connected\n";

pub async fn start(listener: TcpListener) -> std::io::Result<()> {
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted connection from: {}", addr);
        tokio::spawn(async move {
            manage_stream(stream).await;
        });
    }
}

async fn manage_stream(stream: TcpStream) {
    println!("New connection established.");

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let (tx, mut rx) = mpsc::channel::<String>(32);
    let (keepalive_tx, mut keepalive_rx) = mpsc::channel(1);
    let _ = keepalive_tx.send(()).await;

    let writer_task = tokio::spawn(async move {
        if writer.write_all(CONNECTED_MSG.as_bytes()).await.is_err() {
            eprintln!("Failed to send initial Connected message.");
            return;
        }

        while let Some(message) = rx.recv().await {
            if writer.write_all(message.as_bytes()).await.is_err() {
                eprintln!("Failed to write to socket; closing connection.");
                break;
            }
        }
        let _ = writer.shutdown().await;
        println!("Writer task finished.");
    });

    let tx_timer = tx.clone();
    let keep_alive_task = tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(15));
        loop {
            interval.tick().await;
            if keepalive_rx.try_recv().is_err() {
                break;
            }
            if tx_timer.send(KEEP_ALIVE_MSG.to_string()).await.is_err() {
                break;
            }
        }
        println!("Keep-alive task finished.");
    });

    let tx_reader = tx.clone();
    let mut line = String::new();
    let reader_loop_result = async {
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    println!("Connection closed by peer.");
                    break Ok(());
                }
                Ok(_) => {
                    let received_message = line.trim();
                    if received_message == "action:keepAlive" {
                        if tx_reader.send(KEEP_ALIVE_ACK_MSG.to_string()).await.is_err() {
                            break Err(());
                        }
                    } else if received_message == "action:keepAliveAck" {
                        if keepalive_tx.send(()).await.is_err() {
                            break Err(());
                        }
                    } else if !received_message.is_empty() {
                        println!("Received other message: {}", received_message);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from socket; closing connection: {}", e);
                    break Err(());
                }
            }
        }
    };
    let ws_handle_task = async {
            /* Code for handling Websocket */
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    let input = line.trim().to_string();
                    if input == "quit" {
                        break;
                    }
                    if tx.send(input).await.is_err() {
                        break;  // Channel closed
                    }
                }
                Err(e) => {
                    eprintln!("Input error: {}", e);
                    break;
                }
            }
        }
        println!("Input handler shutting down");
    };


    tokio::select! {
        res = reader_loop_result => {
            match res {
                Ok(_) => println!("Reader loop finished gracefully."),
                Err(_) => println!("Reader loop finished due to error or closed channel."),
            }
        },
        _ = writer_task => {
            println!("Writer task completed (possibly due to error or closed channel).");
        },
        _ = keep_alive_task => {
            println!("Keep-alive task completed (normally exits when channel closes).");
        }
        _ = ws_handle_task => {
            println!("Websocket handle completed");
        }

    }

    drop(tx_reader);

    println!("Connection handler finished.");
}
