use remotro::Remotro;
use log;

#[tokio::main]
async fn main() {
    env_logger::init(); // Initialize the logger

    let remotro = Remotro::host("127.0.0.1", 34143).await.unwrap();
    log::info!("Remotro hosted on 127.0.0.1:34143");
    loop {
        let game = remotro.accept().await.unwrap();
        log::info!("Game at {} connected", game.addr());
        loop {}
    }
}
