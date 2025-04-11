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
        log::info!("Game at {} connected", game.addr());
        let mut delay = interval(Duration::from_secs(5));
        loop {
            delay.tick().await;
            match game.send_message("test message").await {
                Ok(_) => continue,
                Err(e) => {
                    log::error!("{e}"); // probably a better way to handle this
                    break;
                },
            }
        }
    }
}
