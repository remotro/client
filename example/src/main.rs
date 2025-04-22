use remotro::{balatro::{menu::{Deck, Stake}, Screen}, Remotro};
use log;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Host a TCP socket
    let mut remotro = Remotro::host("127.0.0.1", 34143).await.unwrap();
    log::info!("Remotro hosted on 127.0.0.1:34143");
    loop {
        // Wait for a Game to connect
        let mut balatro = remotro.accept().await.unwrap();

        // Check current screen in Game
        let screen = balatro.screen().await.unwrap();
        match screen {
            Screen::Menu(menu) => {
                let mut select_blind = menu.new_run(Deck::Anaglyph, Stake::White, None).await.unwrap();
            }
            _ => {
                // Do another thing
            }
        }
        loop {}
    }
}
