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
        loop{
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
                }
                Screen::Blinds(blinds) => {
                    println!("Small blind: {:?}", blinds.small());
                    println!("Big blind: {:?}", blinds.big());
                    println!("Boss blind: {:?}", blinds.boss());
                    println!("Select or skip the blind:");
                    let mut user_input = String::new();
                    std::io::stdin()
                        .read_line(&mut user_input)
                        .expect("Failed to read line from stdin");
                    match user_input.trim().to_lowercase().as_str() {
                        "select" => {
                            println!("Selecting blind");
                            blinds.select().await.unwrap();
                            }
                         "skip" => {
                            println!("Skipping blind");
                            blinds.skip().await.unwrap();
                            }
                         _ => {
                            println!("Invalid input. Please enter Select or Skip.");
                        }
                    }
                }
                _ => {
                    log::error!("(currently) Unimplemented state");
                }
            }
        }
    }
}
