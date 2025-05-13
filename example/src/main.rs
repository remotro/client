use log::{error, info};
use remotro::{
    balatro::{
        menu::{Deck, Stake},
        play::{DiscardResult, PlayResult},
        Screen
    }, Remotro
};
use std::str::FromStr;

fn get_input<T: FromStr<Err = String>>(prompt: &str) -> T {
    loop {
        println!("{prompt}");
        let mut item = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut item) {
            error!("{e}");
            continue;
        }
        match item.parse() {
            Ok(item) => return item,
            Err(e) => {
                error!("{e}");
                continue;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // Host a TCP socket
    let mut remotro = Remotro::host("127.0.0.1", 34143).await.unwrap();
    info!("Remotro hosted on 127.0.0.1:34143");

    loop {
        info!("Waiting for connection");
        // Wait for a Game to connect
        let mut balatro = match remotro.accept().await {
            Ok(b) => {
                info!("New connection accepted");
                b
            }
            Err(e) => {
                error!("Connection Failed: {e}");
                continue;
            }
        };
        loop {
            // Check current screen in Game
            match balatro.screen().await {
                Ok(screen) => match screen {
                    Screen::Menu(menu) => {
                        println!("Main Menu:");
                        // Prompt the user to select Deck
                        let deck: Deck = get_input("Select Deck:");
                        // Prompt the user to select Stake
                        let stake: Stake = get_input("Select stake");
                        menu.new_run(deck, stake, None).await.unwrap();
                    }
                    Screen::SelectBlind(blinds) => {
                        println!("Blinds:");
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
                                if let Err(e) = blinds.skip().await {
                                    error!("{e}");
                                }
                            }
                            _ => {
                                println!("Invalid input. Please enter Select or Skip.");
                            }
                        }
                    }
                    Screen::Play(play) => {
                        println!("Play:");
                        println!("Hand: {:?}", play.hand());
                        println!("Blind: {:?}", play.blind());
                        println!("Score: {}", play.score());
                        println!("Hands: {}", play.hands());
                        println!("Discards: {}", play.discards());
                        println!("Money: ${}",play.money());
                        println!("Select, Play, or Discard cards:");
                        let mut user_input = String::new();
                        std::io::stdin()
                            .read_line(&mut user_input)
                            .expect("Failed to read line from stdin");
                        match user_input.trim().to_lowercase().as_str() {
                            "select" => {
                                println!("Select cards:");
                                let mut user_input = String::new();
                                std::io::stdin()
                                    .read_line(&mut user_input)
                                    .expect("Failed to read line from stdin");
                                let indices: Vec<u32> = user_input
                                    .trim()
                                    .split_whitespace()
                                    .map(|s| s.parse().unwrap())
                                    .collect();
                                if let Err(e) = play.click(&indices).await {
                                    println!("{e}");
                                }
                            },
                            "play" => {
                                let result = play.play().await;
                                match result {
                                    Ok(PlayResult::Again(play)) => {
                                        println!("Must play again");
                                    },
                                    Ok(PlayResult::RoundOver(overview)) => {
                                        println!("Round over");
                                        println!("Total money: {}", overview.total_money());
                                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                                        let result = overview.cash_out().await;
                                        match result {
                                            Ok(_) => println!("Cash out successful"),
                                            Err(e) => println!("{e}"),
                                        }
                                        break;
                                    },
                                    Ok(PlayResult::GameOver(_)) => {
                                        println!("Game over");
                                        break;
                                    },
                                    Err(e) => println!("{e}"),
                                }
                            },
                            "discard" => {
                                let result = play.discard().await;
                                match result {
                                    Ok(DiscardResult::Again(play)) => {
                                        println!("Must discard again");
                                    },
                                    Ok(DiscardResult::GameOver(_)) => {
                                        println!("Game over");
                                        break;
                                    },
                                    Err(e) => println!("{e}"),
                                }
                            },
                            _ => {
                                println!("Invalid input. Please enter Play, Select, or Discard.");
                            }
                        }
                    }
                    Screen::Shop(shop) => {
                        println!("Shop");
                        println!("Items: {:?}", shop.main_cards());
                        println!("Vouchers: {:?}", shop.vouchers());
                        println!("Boosters: {:?}", shop.boosters());
                    }
                },
                Err(e) => {
                    error!("Connection Failed: {e}");
                    break;
                }
            }
        }
    }
}
