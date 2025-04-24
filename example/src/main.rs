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
                // Prompt the user to select Deck
                println!("Select a deck:");
                let mut deck = String::new();
                std::io::stdin()
                    .read_line(&mut deck)
                    .expect("Failed to read line from stdin");
                let deck_bundle = format!("{{ \"{}\": null }}", deck.trim());
                let deck: Deck = serde_json::from_str(&deck_bundle).unwrap();
                
                // Prompt the user to select Stake
                println!("Select a stake:");
                let mut stake = String::new();
                std::io::stdin()
                    .read_line(&mut stake)
                    .expect("Failed to read line from stdin");
                let stake_bundle = format!("{{ \"{}\": null }}", stake.trim());
                let stake: Stake = serde_json::from_str(&stake_bundle).unwrap();

                let select_blind = menu.new_run(deck, stake, None).await.unwrap();
                println!("Small blind: {:?}", select_blind.small());
                println!("Big blind: {:?}", select_blind.big());
                println!("Boss blind: {:?}", select_blind.boss());
            }
            _ => {
                log::error!("(currently) Unimplemented state");
                // Do another thing
            }
        }
        loop {}
    }
}
