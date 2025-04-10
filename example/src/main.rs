use remotro::Remotro;
use log;
use tokio::time::{Duration,interval};

#[tokio::main]
async fn main() {
    env_logger::init(); // Initialize the logger

    let remotro = Remotro::host("127.0.0.1", 34143).await.unwrap();
    log::info!("Remotro hosted on 127.0.0.1:34143");
    loop {
        let game = remotro.accept().await.unwrap();
        log::info!("Game at {} connected", game.stream.addr());
        let mut time = interval(Duration::from_secs(5));
        loop {
            let _ = game.stream.send_message("Still open".to_string()).await;
            time.tick().await;
        }
    }
}
