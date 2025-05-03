use log;
use remotro::{
    Remotro,
    balatro::{
        Screen,
        menu::{Deck, Stake},
    },
};

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
                // Let's assume we want to start a new run with the Red Deck on White Stake
                let deck = Deck::Red;
                let stake = Stake::White;
                log::info!("Starting a new run with {:?} deck on {:?} stake in 5 seconds...", deck, stake);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                let select_blind = menu.new_run(deck, stake, None).await.unwrap();
                println!("Small blind: {:?}", select_blind.small());
                println!("Big blind: {:?}", select_blind.big());
                println!("Boss blind: {:?}", select_blind.boss());
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                let play = select_blind.select().await.unwrap();
                println!("Selected hand: {:?}", play.hand());
                let play = play.click(&[0, 1, 2]).await.unwrap();
                println!("Updated hand: {:?}", play.hand());   
            }
            _ => {
                log::error!("(currently) Unimplemented state");
                // Do another thing
            }
        }
        loop {}
    }
}
