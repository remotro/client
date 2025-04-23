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
                // Prompt the user for Y/N input
                log::warn!("Select a deck:");
                let mut deck = String::new();
                std::io::stdin()
                    .read_line(&mut deck)
                    .expect("Failed to read line from stdin");
                let deck_bundle = format!("{{ \"{}\": null }}", deck.trim());
                let deck: Deck = serde_json::from_str(&deck_bundle).unwrap();
            
                log::warn!("Select a stake:");
                let mut stake = String::new();
                std::io::stdin()
                    .read_line(&mut stake)
                    .expect("Failed to read line from stdin");
                let stake_bundle = format!("{{ \"{}\": null }}", stake.trim());
                let stake: Stake = serde_json::from_str(&stake_bundle).unwrap();

                let select_blind = menu.new_run(deck, stake, None).await.unwrap();
                // Prompt the user for Y/N input
                log::info!("Select or skip a blind");

                let mut user_input = String::new();
                std::io::stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read line from stdin");

                // Process the input
                match user_input.trim().to_lowercase().as_str() {
                    "select" => {
                        log::info!("Selecting blind");
                        select_blind.select().await.unwrap();
                    }
                    "skip" => {
                        log::info!("Skipping blind");
                        select_blind.skip().await.unwrap();
                    }
                    _ => {
                        println!("Invalid input. Please enter Y or N.");
                    }
                }
            }
            _ => {
                // Do another thing
            }
        }
        loop {}
    }
}
