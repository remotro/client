use tokio::net::TcpListener;
mod game;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let _ = tokio::spawn(async move {   
        let tcp_listener = TcpListener::bind("127.0.0.1:34143").await?;
        game::start(tcp_listener).await
    }).await?;
    Ok(())
}
