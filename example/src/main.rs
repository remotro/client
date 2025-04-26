use log;
use log::error;
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
                    let deck: Deck = loop {
                        println!("Select a deck:");
                        let mut deck = String::new();
                        if let Err(e) = std::io::stdin().read_line(&mut deck) {
                            error!("{e}");
                            continue;
                        }
                        let deck_bundle = format!("{{ \"{}\": null }}", deck.trim());
                        match serde_json::from_str(&deck_bundle) {
                            Ok(deck) => break deck,
                            Err(e) => {
                                error!("{e}");
                                continue;
                            },
                        }
                    };
                    
                    // Prompt the user to select Stake
                    let stake: Stake = loop {
                        println!("Select a stake:");
                        let mut stake = String::new();
                        if let Err(e) = std::io::stdin().read_line(&mut stake) {
                            error!("{e}");
                            continue;
                        }
                        let stake_bundle = format!("{{ \"{}\": null }}", stake.trim());

                        match serde_json::from_str(&stake_bundle) {
                            Ok(stake) => break stake,
                            Err(e) => error!("{e}"),
                        }
                    };

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
