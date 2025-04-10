use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
    sync::Notify,
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
        // Notify for keep-alive acknowledgements
        let keepalive_ack_notify = Arc::new(Notify::new());
        // Channel for receiving messages from the reader task
        let (reader_tx, reader_rx) = mpsc::channel::<String>(32);

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
        let keepalive_ack_notify_clone = keepalive_ack_notify.clone(); // Clone Arc<Notify>
        let keepalive_handle = tokio::spawn(async move {
            let mut keepalive_timeout = interval(Duration::from_secs(KEEPALIVE_TIME_SECS));
            let mut keepalive_retries = 1;
            // Consume the first tick immediately as interval fires instantly
            keepalive_timeout.tick().await;
            // Start notified so the first loop iteration sends a ping immediately.
            keepalive_ack_notify_clone.notify_one();

            loop {
                tokio::select! {
                    biased; // Prioritize checking for closed channel

                     _ = writer_tx_keepalive.closed() => {
                        info!("[{}] Writer channel closed externally. Keep-alive task stopping.", addr);
                        break;
                    }

                    // Wait until notified (meaning an ack was received)
                    _ = keepalive_ack_notify_clone.notified() => {
                        // Ack was received since the last check. Reset retries.
                        debug!("[{}] Keep-alive ack received, resetting retries.", addr);
                        keepalive_retries = 1;
                        // Permit is consumed. Loop will continue to wait for timeout.
                    }

                    // Wait for the timeout interval
                    _ = keepalive_timeout.tick() => {
                        // Timeout elapsed. Time to send a ping.
                        // `keepalive_retries` reflects state since last successful ack.
                        if keepalive_retries > KEEPALIVE_MAX_RETRIES {
                            warn!("[{}] Keep-alive failed after {} retries (no ack received since last check). Closing connection.", addr, KEEPALIVE_MAX_RETRIES);
                            drop(writer_tx_keepalive); // Signal writer task to exit by dropping sender
                            break;
                        }

                        // Send keep-alive ping
                        trace!("[{}] Sending keep-alive ping (Retry {}/{})", addr, keepalive_retries, KEEPALIVE_MAX_RETRIES);
                        if writer_tx_keepalive.send(KEEP_ALIVE_MSG.to_string()).await.is_err() {
                            // Writer task likely closed if send fails
                            info!("[{}] Writer channel closed. Keep-alive task stopping.", addr);
                            break;
                        }

                        // Increment retries for the *next* check. If an ack comes in before the next tick,
                        // retries will be reset by the `notified()` branch.
                        keepalive_retries += 1;
                    }
                }
            }
            info!("[{}] Keep-alive task finished.", addr);
        });

        // --- Reader Task ---
        let writer_tx_reader_ack = writer_tx.clone(); // For sending keepAliveAck
        let keepalive_ack_notify_reader_clone = keepalive_ack_notify.clone(); // Clone Arc<Notify>
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
                            m if m == KEEP_ALIVE_MSG.trim() => {
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
                            m if m == KEEP_ALIVE_ACK_MSG.trim() => {
                                trace!("[{}] Received keepAliveAck.", addr);
                                // Signal keep-alive task that ack was received
                                keepalive_ack_notify_reader_clone.notify_one();
                                trace!("[{}] Notified keep-alive task.", addr);
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
            // Reader task is ending, close associated channels/signals
            drop(reader_tx); // Signal owner no more messages
            drop(writer_tx_reader_ack); // Signal writer (potentially)
            // No need to explicitly drop Arc<Notify>, happens automatically when all clones are dropped
            info!("[{}] Reader task finished.", addr);
        });

        Self {
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
}

impl Drop for ManagedTcpStream {
    fn drop(&mut self) {
        info!("[{}] Dropping ManagedTcpStream, shutting down tasks.", self.addr);
        // Aborting tasks is generally the way to stop them immediately.
        // Dropping the writer_tx handle signals the writer task to exit gracefully.
        // The reader and keepalive tasks check for channel closures.
        self.writer_handle.abort();
        self.keepalive_handle.abort();
        self.reader_handle.abort();
    }
}
