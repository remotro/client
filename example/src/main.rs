use log::{error, info};
use remotro::{
    balatro::{
        boosters::{BoosterCard, BoosterPackKind, Open, OpenWithHand},
        hud::Hud,
        menu::{Deck, Stake, Seed},
        play::{DiscardResult, PlayResult},
        shop::{BoosterPack, BoughtBooster, MainCard},
        Balatro,
        CurrentScreen
    },
    Remotro
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

macro_rules! as_variant {
    ($screen:expr, $variant:path) => {
        if let $variant(screen) = $screen {
            screen
        } else {
            println!("Not in variant");
            return;
        }
    }
}

async fn quickrun(mut balatro: Balatro) {
    let screen = as_variant!(balatro.screen().await.unwrap(), CurrentScreen::Menu);
    println!("< starting run >");
    let mut blind_select = screen.new_run(Deck::Red, Stake::White, None).await.unwrap();
    let mut play_result = blind_select.select().await.unwrap().click(&[0, 1, 2]).await.unwrap();
    println!("Poker hand: {:?}", play_result.poker_hand());
}

fn print_hud<'a, T: Hud<'a>>(hud: &T) {
    println!("  HUD");
    println!("  Hands: {:?}", hud.hands());
    println!("  Discards: {:?}", hud.discards());
    println!("  Money: {:?}", hud.money());
    println!("  Jokers: {:?}", hud.jokers());
    println!("  Consumables: {:?}", hud.consumables());
    println!("  Round: {:?}", hud.round());
    println!("  Ante: {:?}", hud.ante());
    println!("  Run Info: {:?}", hud.run_info());
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
        // prompt user to select quickrun or normal run
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line from stdin");
        match user_input.trim().to_lowercase().as_str() {
            "quickrun" => {
                quickrun(balatro).await;
                return;
            }
            "normalrun" => {
            }
            _ => {
                error!("Invalid input. Please enter quickrun or normalrun.");
                continue;
            }
        }
        loop {
            // Check current screen in Game
            match balatro.screen().await {
                Ok(screen) => match screen {
                    CurrentScreen::Menu(menu) => {
                        println!("Main Menu:");
                        // Prompt the user to select Deck
                        let deck: Deck = get_input("Select Deck:");
                        // Prompt the user to select Stake
                        let stake: Stake = get_input("Select stake");
                        menu.new_run(deck, stake, None).await.unwrap();
                    }
                    CurrentScreen::SelectBlind(blinds) => {
                        println!("Blinds:");
                        print_hud(&blinds);
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
                    CurrentScreen::Play(play) => {
                        println!("Play:");
                        println!("Hand: {:?}", play.hand());
                        println!("Blind: {:?}", play.blind());
                        println!("Score: {}", play.score());
                        print_hud(&play);
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
                                        print_hud(&overview);
                                        println!("Total money: {}", overview.total_earned());
                                        println!("Earnings: {:?}", overview.earnings());
                                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                                        let result = overview.cash_out().await;
                                        match result {
                                            Ok(_) => println!("Cash out successful"),
                                            Err(e) => println!("{e}"),
                                        }
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
                    CurrentScreen::Shop(mut shop) => {
                        println!("Shop");
                        println!("Items: {:?}", shop.main_cards());
                        println!("Vouchers: {:?}", shop.vouchers());
                        println!("Boosters: {:?}", shop.boosters());
                        println!("Select a main item to buy");
                        let mut user_input = String::new();
                        std::io::stdin()
                            .read_line(&mut user_input)
                            .expect("Failed to read line from stdin");
                        let index: u32 = user_input.trim().parse().unwrap();
                        shop = shop.buy_main(index as u8).await.unwrap();
                        print_hud(&shop);
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
