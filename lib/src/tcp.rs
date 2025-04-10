use std::net::SocketAddr;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
    task::JoinHandle,
    time::{Duration, interval},
};
use log::{info, warn, error, debug, trace};

const KEEP_ALIVE_MSG: &str = "action:keepAlive\n";
const KEEP_ALIVE_ACK_MSG: &str = "action:keepAliveAck\n";
const CONNECTED_MSG: &str = "Connected\n";

const KEEPALIVE_MAX_RETRIES: i32 = 5;
const KEEPALIVE_TIME_SECS: u64 = 5;

/// Manages accepting TCP connections.
pub struct ManagedTcpListener {
    tcp_listener: TcpListener,
}

impl ManagedTcpListener {
    pub async fn bind(host: impl AsRef<str>, port: u16) -> Result<Self, std::io::Error> {
        let tcp_listener = TcpListener::bind(format!("{}:{}", host.as_ref(), port)).await?;
        Ok(Self { tcp_listener })
    }

    /// Accepts a new TCP connection and returns a ManagedTcpStream.
    pub async fn accept(&self) -> std::io::Result<ManagedTcpStream> {
        let (stream, addr) = self.tcp_listener.accept().await?;
        info!("[{}] Accepted connection", addr);
        Ok(ManagedTcpStream::new(stream, addr).await)
    }
}

/// Represents a managed TCP connection with background tasks for keep-alive and I/O.
#[derive(Debug)]
pub struct ManagedTcpStream {
    writer_tx: Sender<String>,
    reader_rx: Receiver<String>,
    writer_handle: JoinHandle<()>,
    keepalive_handle: JoinHandle<()>,
    reader_handle: JoinHandle<()>,
    addr: SocketAddr,
}

impl ManagedTcpStream {
    /// Internal constructor to set up tasks and channels.
    async fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        // Channel for sending messages to the writer task
        let (writer_tx, mut writer_rx) = mpsc::channel::<String>(32);
        // Channel for receiving keep-alive acknowledgements
        let (keepalive_ack_tx, mut keepalive_ack_rx) = mpsc::channel::<()>(1);
        // Channel for receiving messages from the reader task
        let (reader_tx, reader_rx) = mpsc::channel::<String>(32);

        // Initial keep-alive ack signal - start enabled
        let _ = keepalive_ack_tx.send(()).await;

        // --- Writer Task ---
        let writer_handle = tokio::spawn(async move {
            // Send initial connection message
            if writer.write_all(CONNECTED_MSG.as_bytes()).await.is_err() {
                error!("[{}] Failed to send initial Connected message.", addr);
                let _ = writer.shutdown().await; // Attempt graceful shutdown
                return;
            }

            // Process outgoing messages
            while let Some(message) = writer_rx.recv().await {
                if writer.write_all(message.as_bytes()).await.is_err() {
                    error!("[{}] Failed to write to socket; closing writer task.", addr);
                    break; // Exit loop on write error
                }
            }
            // Shutdown writer when the writer_tx channel is closed
            let _ = writer.shutdown().await;
            info!("[{}] Writer task finished.", addr);
        });

        // --- Keep-Alive Task ---
        let writer_tx_keepalive = writer_tx.clone();
        let keepalive_handle = tokio::spawn(async move {
            let mut keepalive_timeout = interval(Duration::from_secs(KEEPALIVE_TIME_SECS));
            let mut keepalive_retries = 1;
            // Consume the first tick immediately, as it fires instantly
            keepalive_timeout.tick().await;
            loop {
                tokio::select! {
                    _ = keepalive_timeout.tick() => {
                        // Check if ack was received since last tick
                        if keepalive_ack_rx.try_recv().is_err() { // No ack received
                            if keepalive_retries >= KEEPALIVE_MAX_RETRIES {
                                warn!("[{}] Keep-alive failed after {} retries. Closing connection.", addr, KEEPALIVE_MAX_RETRIES);
                                // Closing writer_tx_keepalive signals writer task to exit
                                drop(writer_tx_keepalive);
                                break;
                            }
                             warn!("[{}] Keep-alive check failed (retry {}/{})", addr, keepalive_retries, KEEPALIVE_MAX_RETRIES);
                            keepalive_retries += 1;
                        } else { // Ack received
                            // Reset retries only if an ack was explicitly received
                             debug!("[{}] Keep-alive ack received, resetting retries.", addr);
                            keepalive_retries = 1;
                        }

                        // Send keep-alive ping
                        if writer_tx_keepalive.send(KEEP_ALIVE_MSG.to_string()).await.is_err() {
                            // Writer task likely closed, exit keep-alive task
                            info!("[{}] Writer channel closed. Keep-alive task stopping.", addr);
                            break;
                        } else {
                            trace!("[{}] Sent keep-alive ping.", addr);
                        }
                    }
                    // Allow task to exit if writer channel closes externally
                     _ = writer_tx_keepalive.closed() => {
                        info!("[{}] Writer channel closed externally. Keep-alive task stopping.", addr);
                        break;
                    }
                }
            }
            info!("[{}] Keep-alive task finished.", addr);
        });

        // --- Reader Task ---
        let writer_tx_reader_ack = writer_tx.clone(); // For sending keepAliveAck
        let reader_handle = tokio::spawn(async move {
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        info!("[{}] Connection closed by peer (EOF).", addr);
                        break; // EOF
                    }
                    Ok(_) => {
                        let received_message = line.trim();
                        match received_message {
                            "action:keepAlive" => {
                                trace!("[{}] Received keepAlive ping.", addr);
                                // Respond with keepAliveAck
                                if writer_tx_reader_ack
                                    .send(KEEP_ALIVE_ACK_MSG.to_string())
                                    .await
                                    .is_err()
                                {
                                    error!(
                                        "[{}] Failed to send keepAliveAck: writer channel closed.",
                                        addr
                                    );
                                    break; // Stop reader if we can't ack
                                }
                                trace!("[{}] Sent keepAliveAck.", addr);
                            }
                            "action:keepAliveAck" => {
                                trace!("[{}] Received keepAliveAck.", addr);
                                // Signal keep-alive task that ack was received
                                if keepalive_ack_tx.try_send(()).is_err() {
                                    // This might happen if the keepalive task already stopped, which is okay.
                                    debug!(
                                        "[{}] Keep-alive ack channel closed, task likely stopped.",
                                        addr
                                    );
                                }
                            }
                            "" => {} // Ignore empty lines
                            _ => {
                                // Send other messages to the ManagedTcpStream owner
                                debug!("[{}] Received message: {}", addr, received_message);
                                if reader_tx.send(received_message.to_string()).await.is_err() {
                                    error!(
                                        "[{}] Failed to send message to owner: reader channel closed.",
                                        addr
                                    );
                                    break; // Stop reader if owner is no longer listening
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "[{}] Failed to read from socket: {}. Closing reader task.",
                            addr, e
                        );
                        break; // Error
                    }
                }
            }
            // Reader task is ending, close associated channels
            drop(reader_tx); // Signal owner no more messages
            drop(writer_tx_reader_ack); // Signal writer (potentially)
            drop(keepalive_ack_tx); // Signal keepalive task
            info!("[{}] Reader task finished.", addr);
        });

        Self {
            // Changed from Ok(ManagedTcpStream{...})
            writer_tx,
            reader_rx,
            writer_handle,
            keepalive_handle,
            reader_handle,
            addr,
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Sends a message over the TCP connection.
    /// Returns an error if the connection's writer task has stopped.
    pub async fn send_message(
        &self,
        message: String,
    ) -> Result<(), mpsc::error::SendError<String>> {
        // Ensure message ends with a newline if not already present
        let formatted_message = if message.ends_with('\n') {
            message
        } else {
            format!("{}\n", message)
        };
        self.writer_tx.send(formatted_message).await
    }

    /// Receives a message from the TCP connection.
    /// Returns `Ok(Some(String))` if a message is received.
    /// Returns `Ok(None)` if the connection was closed gracefully by the peer or an error occurred in the reader task.
    pub async fn recv_message(&mut self) -> Option<String> {
        self.reader_rx.recv().await
    }

    /// Aborts the background tasks associated with this stream.
    /// This is usually called automatically when the stream is dropped.
    pub fn shutdown(&self) {
        info!("[{}] Shutting down ManagedTcpStream tasks.", self.addr);
        // Aborting tasks is generally the way to stop them immediately.
        // Dropping the writer_tx handle signals the writer task to exit gracefully.
        // The reader and keepalive tasks check for channel closures.
        self.writer_handle.abort();
        self.keepalive_handle.abort();
        self.reader_handle.abort();
    }
}

impl Drop for ManagedTcpStream {
    fn drop(&mut self) {
        info!(
            "[{}] Dropping ManagedTcpStream, shutting down tasks.",
            self.addr
        );
        self.shutdown();
    }
}
