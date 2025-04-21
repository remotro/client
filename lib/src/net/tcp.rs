use serde::Serialize;
use std::borrow::Cow;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::{mpsc, oneshot},
    task::JoinHandle,
    time::{Instant, sleep},
};

use super::Error;
use super::protocol::Packet; // Assuming Error is in super

use log::{debug, error, info, trace, warn};
use serde::de::DeserializeOwned;
use std::time::Duration;

// --- Constants for Heartbeat and Connection Logic ---

/// Size of the MPSC channels used for communication between the main struct and the background task.
const CHANNEL_BUFFER_SIZE: usize = 32;
/// Duration of inactivity (no packets received or sent) before a ping is sent.
const INACTIVITY_TIMEOUT_SECS: u64 = 7;
/// Duration to wait for a response (any packet) after sending a ping before retrying.
const PING_RESPONSE_TIMEOUT_SECS: u64 = 3;
/// Maximum number of ping retries before closing the connection due to timeout.
const MAX_PING_RETRIES: u8 = 3;
/// The exact string format for a ping packet (including delimiter).
const PING_PACKET: &str = "ping!";
/// The exact string format for a pong packet (including delimiter).
const PONG_PACKET: &str = "pong!";
/// A practically infinite duration used to disable timers initially.
const FOREVER_DURATION: Duration = Duration::from_secs(u64::MAX);

/// Represents a TCP stream with an associated background task handling
/// raw I/O, packet framing (kind!body\\n), and a heartbeat mechanism.
///
/// Communication between the public methods (`send`, `recv`) and the background task
/// occurs via asynchronous channels.
pub struct TcpStreamExt {
    /// Sends fully formatted packet strings (`kind!body`) to the background task for writing.
    tx_outgoing: mpsc::Sender<String>,
    /// Receives results containing either successfully read and framed packets (`kind!body`)
    /// or errors from the background task.
    rx_incoming: mpsc::Receiver<Result<String, Error>>,
    /// Handle to the background I/O and heartbeat task. Kept primarily to ensure
    /// the task is associated with the lifetime of the struct, though not directly joined.
    _task_handle: JoinHandle<()>,
    /// Oneshot sender used to signal the background task to shut down gracefully.
    /// Held in an Option to allow taking it during Drop.
    close_tx: Option<oneshot::Sender<()>>,
}

impl TcpStreamExt {
    pub fn new(stream: TcpStream) -> Self {
        let (reader_half, writer_half) = stream.into_split();
        let reader = BufReader::new(reader_half);
        let writer = BufWriter::new(writer_half);

        let (tx_outgoing, rx_outgoing) = mpsc::channel::<String>(CHANNEL_BUFFER_SIZE);
        let (tx_incoming, rx_incoming) =
            mpsc::channel::<Result<String, Error>>(CHANNEL_BUFFER_SIZE);
        let (close_tx, close_rx) = oneshot::channel::<()>();

        let task_handle = tokio::spawn(run_connection(
            reader,
            writer,
            rx_outgoing,
            tx_incoming.clone(),
            close_rx,
        ));

        Self {
            tx_outgoing,
            rx_incoming,
            _task_handle: task_handle,
            close_tx: Some(close_tx),
        }
    }

    pub async fn send<T: Serialize + Packet>(&mut self, msg: T) -> Result<(), Error> {
        let body = serde_json::to_string(&msg)?;
        let packet_str = format!("{}!{}", T::kind(), body);
        self.tx_outgoing.send(packet_str).await.map_err(|_| {
            Error::ChannelError(Cow::Borrowed("Failed to send packet to background task"))
        })?;
        Ok(())
    }

    pub async fn recv<R: DeserializeOwned + Packet>(&mut self) -> Result<R, Error> {
        let received_result = self
            .rx_incoming
            .recv()
            .await
            .ok_or(Error::ConnectionClosed)?;

        let buf = received_result?;

        let mut split = buf.splitn(2, '!');
        let kind = split
            .next()
            .ok_or(Error::Message(Cow::Borrowed("Received packet has no kind")))?;
        let body = split
            .next()
            .ok_or(Error::Message(Cow::Borrowed("Received packet has no body")))?;

        if kind != R::kind() {
            return Err(Error::Message(Cow::Owned(format!(
                "Expected response kind {}, got {}",
                R::kind(),
                kind
            ))));
        }

        let response: R = serde_json::from_str(body)?;
        Ok(response)
    }
}

impl Drop for TcpStreamExt {
    fn drop(&mut self) {
        if let Some(sender) = self.close_tx.take() {
            let _ = sender.send(());
        }
    }
}

/// The core background task that handles TCP reading, writing, packet framing,
/// and the ping/pong heartbeat mechanism.
///
/// It runs in a loop, using `tokio::select!` to concurrently manage:
/// 1. Receiving outgoing packets from `TcpStreamExt::send` via `rx_outgoing`.
/// 2. Reading incoming data from the `TcpStream` (`reader`).
/// 3. Handling ping responses and forwarding other data via `tx_incoming`.
/// 4. Tracking inactivity and sending pings.
/// 5. Tracking ping responses and handling retries/timeouts.
/// 6. Listening for a shutdown signal via `close_rx`.
async fn run_connection(
    mut reader: BufReader<OwnedReadHalf>,
    mut writer: BufWriter<OwnedWriteHalf>,
    mut rx_outgoing: mpsc::Receiver<String>, // Messages to send from Self::send
    tx_incoming: mpsc::Sender<Result<String, Error>>, // Framed messages or errors back to Self::recv
    mut close_rx: oneshot::Receiver<()>,              // Signal to close from Drop
) {
    info!("Connection task started.");
    // Buffer for reading lines from the socket.
    let mut line_buf = String::new();
    // Durations for timers, loaded from constants.
    let inactivity_timeout = Duration::from_secs(INACTIVITY_TIMEOUT_SECS);
    let ping_response_timeout = Duration::from_secs(PING_RESPONSE_TIMEOUT_SECS);

    // --- Timers ---
    // Timer for detecting inactivity (no sends or receives).
    let inactivity_timer = sleep(inactivity_timeout);
    // Timer for detecting lack of response after a ping has been sent.
    // Initialized to sleep forever, effectively disabling it until the first ping.
    let ping_timer = sleep(FOREVER_DURATION);

    // Timers must be pinned to be used in `select!` as `sleep` doesn't produce `Unpin` futures.
    tokio::pin!(inactivity_timer);
    tokio::pin!(ping_timer);

    // --- State ---
    // Counter for consecutive pings sent without receiving *any* packet in response.
    let mut pings_sent_without_response = 0;

    loop {
        tokio::select! {
            // `biased;` ensures that the shutdown signal is checked first in each loop iteration,
            // allowing for prompt termination when `TcpStreamExt` is dropped.
            biased;

            // 1. Check for shutdown signal from `TcpStreamExt::drop`.
            _ = &mut close_rx => {
                // Got close signal.
                info!("Received shutdown signal.");
                break;
            }

            // 2. Read data from the TCP stream.
            read_result = reader.read_line(&mut line_buf) => {
                match read_result {
                    Ok(0) => { // EOF - Connection closed cleanly by peer.
                        info!("Connection closed by peer (EOF).");
                        let _ = tx_incoming.send(Err(Error::ConnectionClosed)).await;
                        break;
                    }
                    Ok(bytes_read) => { // Received some data.
                        trace!("Read {} bytes.", bytes_read);
                        // Any received data resets inactivity and ping state.
                        inactivity_timer.as_mut().reset(Instant::now() + inactivity_timeout);
                        // No need to reset ping_timer here; the `if` condition in select!
                        // prevents it from firing when pings_sent_without_response is 0.
                        if pings_sent_without_response > 0 {
                            debug!("Activity detected, resetting ping retry count.");
                        }
                        pings_sent_without_response = 0;

                        // Process the received line (remove trailing newline).
                        let received_line = line_buf.trim_end();
                        trace!("Received line: '{}'", received_line);

                        if received_line == PING_PACKET {
                            // Received a ping, send a pong back immediately.
                            // Note: We add the newline back for the write.
                            debug!("Received PING, sending PONG.");
                            if let Err(e) = writer.write_all(format!("{}\n", PONG_PACKET).as_bytes()).await {
                                error!("Failed to send PONG: {}", e);
                                let _ = tx_incoming.send(Err(e.into())).await; // Report write error upstream.
                                break;
                            }
                            if let Err(e) = writer.flush().await {
                                error!("Failed to flush PONG: {}", e);
                                let _ = tx_incoming.send(Err(e.into())).await; // Report flush error upstream.
                                break;
                            }
                            trace!("PONG sent successfully.");
                            // Don't forward the internal "ping!" packet upstream to `recv`.
                        } else {
                            // Check if it's a PONG packet. If so, log it but don't forward.
                            if received_line == PONG_PACKET {
                                debug!("Received PONG.");
                                // PONG is primarily for acknowledging the connection is alive,
                                // no need to forward it to the application layer via `recv`.
                            } else {
                                // Received a regular data packet.
                                // Send the raw framed packet string upstream via the channel.
                                debug!("Received data packet, forwarding upstream.");
                                let owned_line = received_line.to_string();
                                if tx_incoming.send(Ok(owned_line)).await.is_err() {
                                    // Upstream receiver (`TcpStreamExt::recv`) has been dropped. Connection is useless.
                                    info!("Upstream receiver closed, shutting down connection task.");
                                    break;
                                }
                                trace!("Forwarded packet upstream.");
                            }
                        }
                        // Clear the buffer for the next read operation.
                        line_buf.clear();
                    }
                    Err(e) => { // Error during read.
                        error!("TCP read error: {}", e);
                        let _ = tx_incoming.send(Err(e.into())).await; // Report IO error upstream.
                        break;
                    }
                }
            }

            // 3. Send an outgoing packet requested by `TcpStreamExt::send`.
            Some(packet_str) = rx_outgoing.recv() => {
                trace!("Received packet from upstream to send: '{}'", packet_str);
                // Add newline because the reader side uses `read_line`.
                let packet_with_newline = format!("{}\n", packet_str);
                if let Err(e) = writer.write_all(packet_with_newline.as_bytes()).await {
                    error!("TCP write error: {}", e);
                    let _ = tx_incoming.send(Err(e.into())).await; // Report write error upstream.
                    break;
                }
                if let Err(e) = writer.flush().await {
                    error!("TCP flush error: {}", e);
                    let _ = tx_incoming.send(Err(e.into())).await; // Report flush error upstream.
                    break;
                }
                debug!("Successfully sent packet: '{}'", packet_str);
                // Successfully sent data, so reset the inactivity timer.
                inactivity_timer.as_mut().reset(Instant::now() + inactivity_timeout);
                // Sending data also counts as activity, reset ping state if we were waiting.
                // No need to reset ping_timer here; the `if` condition in select!
                // prevents it from firing when pings_sent_without_response is 0.
                pings_sent_without_response = 0; // Reset pings on successful send too
            }

            // 4. Inactivity timer fired.
            _ = &mut inactivity_timer => {
                // No packets sent or received for INACTIVITY_TIMEOUT_SECS.
                // Send the first ping if we aren't already in a ping/pong cycle.
                if pings_sent_without_response == 0 {
                    debug!("Inactivity detected, sending PIN    G (Attempt 1/{})", MAX_PING_RETRIES);
                    if let Err(e) = writer.write_all(format!("{}\n", PING_PACKET).as_bytes()).await {
                        error!("Failed to send PING (Attempt 1): {}", e);
                        let _ = tx_incoming.send(Err(e.into())).await;
                        break;
                    }
                    if let Err(e) = writer.flush().await {
                        error!("Failed to flush PING (Attempt 1): {}", e);
                        let _ = tx_incoming.send(Err(e.into())).await;
                        break;
                    }
                    trace!("PING (Attempt 1) sent successfully.");
                    pings_sent_without_response = 1;
                    // Start the ping response timer.
                    ping_timer.as_mut().reset(Instant::now() + ping_response_timeout);
                    // Reset the inactivity timer as well, sending a ping counts as activity.
                    inactivity_timer.as_mut().reset(Instant::now() + inactivity_timeout);
                }
                 else {
                    // This branch is hit if inactivity timer fires *while* we are already waiting for a pong.
                    // This is normal, the ping_timer branch handles the actual timeout/retry logic.
                    trace!("Inactivity timer fired while waiting for PONG, deferring to ping timer.");
                }
            }

            // 5. Ping response timer fired (only active if pings_sent_without_response > 0).
            _ = &mut ping_timer, if pings_sent_without_response > 0 => {
                warn!("No response received after PING (Attempt {}/{})", pings_sent_without_response, MAX_PING_RETRIES);
                // Waited PING_RESPONSE_TIMEOUT_SECS for *any* packet after sending a ping, but received none.
                if pings_sent_without_response >= MAX_PING_RETRIES {
                    // Exceeded max retries, declare timeout.
                    error!("Ping timeout after {} retries. Closing connection.", MAX_PING_RETRIES);
                    let _ = tx_incoming.send(Err(Error::Timeout)).await;
                    break;
                }

                // Send another ping (retry).
                let next_attempt = pings_sent_without_response + 1;
                debug!("Sending PING (Attempt {}/{})", next_attempt, MAX_PING_RETRIES);
                if let Err(e) = writer.write_all(format!("{}\n", PING_PACKET).as_bytes()).await {
                    error!("Failed to send PING (Attempt {}): {}", next_attempt, e);
                    let _ = tx_incoming.send(Err(e.into())).await;
                    break;
                }
                if let Err(e) = writer.flush().await {
                     error!("Failed to flush PING (Attempt {}): {}", next_attempt, e);
                    let _ = tx_incoming.send(Err(e.into())).await;
                    break;
                }
                trace!("PING (Attempt {}) sent successfully.", next_attempt);

                pings_sent_without_response += 1;
                // Reset the ping response timer for the next retry.
                ping_timer.as_mut().reset(Instant::now() + ping_response_timeout);
                // Also reset the inactivity timer.
                inactivity_timer.as_mut().reset(Instant::now() + inactivity_timeout);
            }

            // `else` branch is required by `select!` when using `biased;`.
            // It's reached if no other branch is ready.
            else => {
                info!("Select loop yielded no active branch, likely due to channel closure. Shutting down.");
                // This might happen if channels are closed unexpectedly.
                // Treat as a reason to shut down the task.
                break;
            }
        }
    }
    // Loop exited (due to error, close signal, or channel closure).
    // Ensure the upstream receiver knows the connection is closed if it hasn't already received an error.
    // Ignore error here, as the receiver might already be dropped.
    info!("Connection task finished.");
    let _ = tx_incoming.send(Err(Error::ConnectionClosed)).await;
}
