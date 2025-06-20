use log::{error, info};
use remotro::{
    balatro::{
        boosters::{Open, OpenWithHand},
        hud::{Hud, RunInfo}, 
        menu::{Deck, Stake}, 
        play::{DiscardResult, PlayResult}, 
        CurrentScreen, OpenPack
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
        match item.trim().parse() {
            Ok(item) => return item,
            Err(e) => {
                error!("{e}");
                continue;
            }
        }
    }
}

fn get_string_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn display_menu() {
    println!("=== MAIN MENU ===");
    println!("Available actions:");
    println!("  new - Start a new run");
    println!("  resume - Resume saved run (if available)");
}

fn display_blinds_menu() {
    println!("\n=== BLIND SELECTION ===");
    println!("Available actions:");
    println!("  select - Select and play the current blind");
    println!("  skip - Skip the current blind");
    println!("  hud - Manage jokers/consumables");
}

fn display_play_menu() {
    println!("\n=== PLAY SCREEN ===");
    println!("Available actions:");
    println!("  select <indices> - Select/deselect cards (e.g., 'select 0 1 2')");
    println!("  play - Play selected cards");
    println!("  discard - Discard selected cards");
    println!("  hud - Manage jokers/consumables");
}

fn display_shop_menu() {
    println!("\n=== SHOP ===");
    println!("Available actions:");
    println!("  buy <type> <index> - Buy item (types: main, voucher, booster)");
    println!("  reroll - Reroll shop contents");
    println!("  leave - Leave shop");
    println!("  hud - Manage jokers/consumables");
}

fn display_overview_menu() {
    println!("\n=== ROUND OVERVIEW ===");
    println!("Available actions:");
    println!("  continue - Continue to shop");
}

fn display_game_over_menu() {
    println!("\n=== GAME OVER ===");
    println!("Available actions:");
    println!("  menu - Return to main menu");
}

fn display_pack_menu() {
    println!("\n=== BOOSTER PACK ===");
    println!("Available actions:");
    println!("  select <index> - Select option by index");
    println!("  click <indices> - Click cards in hand (for packs with hands)");
    println!("  skip - Skip remaining selections");
}

fn display_hud_menu() {
    println!("\n=== HUD MANAGEMENT ===");
    println!("Available actions:");
    println!("  jokers - Manage jokers");
    println!("  consumables - Manage consumables");
    println!("  tags - View tags");
    println!("  back - Return to previous screen");
}

fn display_joker_menu() {
    println!("\n=== JOKER MANAGEMENT ===");
    println!("Available actions:");
    println!("  move <from> <to> - Move joker from position to position");
    println!("  sell <index> - Sell joker at index");
    println!("  back - Return to HUD menu");
}

fn display_consumable_menu() {
    println!("\n=== CONSUMABLE MANAGEMENT ===");
    println!("Available actions:");
    println!("  use <index> - Use consumable at index");
    println!("  move <from> <to> - Move consumable from position to position");
    println!("  sell <index> - Sell consumable at index");
    println!("  back - Return to HUD menu");
}

fn print_run_info(run_info: &RunInfo) {
    println!("\n--- RUN INFO ---");
    println!("Stake: {:?}", run_info.stake);
    
    if !run_info.vouchers_redeemed.is_empty() {
        println!("Vouchers Redeemed: {:?}", run_info.vouchers_redeemed);
    }
    
    println!("\nPoker Hands:");
    let hands = &run_info.poker_hands;
    
    if hands.high_card.played > 0 {
        println!("  High Card: Lv.{} ({} chips, {}x mult) - Played {} times", 
                 hands.high_card.hand.level, hands.high_card.hand.chips, 
                 hands.high_card.hand.mult, hands.high_card.played);
    }
    if hands.pair.played > 0 {
        println!("  Pair: Lv.{} ({} chips, {}x mult) - Played {} times", 
                 hands.pair.hand.level, hands.pair.hand.chips, 
                 hands.pair.hand.mult, hands.pair.played);
    }
    if hands.two_pair.played > 0 {
        println!("  Two Pair: Lv.{} ({} chips, {}x mult) - Played {} times", 
                 hands.two_pair.hand.level, hands.two_pair.hand.chips, 
                 hands.two_pair.hand.mult, hands.two_pair.played);
    }
    if hands.three_of_a_kind.played > 0 {
        println!("  Three of a Kind: Lv.{} ({} chips, {}x mult) - Played {} times", 
                 hands.three_of_a_kind.hand.level, hands.three_of_a_kind.hand.chips, 
                 hands.three_of_a_kind.hand.mult, hands.three_of_a_kind.played);
    }
    if hands.straight.played > 0 {
        println!("  Straight: Lv.{} ({} chips, {}x mult) - Played {} times", 
                 hands.straight.hand.level, hands.straight.hand.chips, 
                 hands.straight.hand.mult, hands.straight.played);
    }
    if hands.flush.played > 0 {
        println!("  Flush: Lv.{} ({} chips, {}x mult) - Played {} times", 
                 hands.flush.hand.level, hands.flush.hand.chips, 
                 hands.flush.hand.mult, hands.flush.played);
    }
    if hands.full_house.played > 0 {
        println!("  Full House: Lv.{} ({} chips, {}x mult) - Played {} times", 
                 hands.full_house.hand.level, hands.full_house.hand.chips, 
                 hands.full_house.hand.mult, hands.full_house.played);
    }
    if hands.four_of_a_kind.played > 0 {
        println!("  Four of a Kind: Lv.{} ({} chips, {}x mult) - Played {} times", 
                 hands.four_of_a_kind.hand.level, hands.four_of_a_kind.hand.chips, 
                 hands.four_of_a_kind.hand.mult, hands.four_of_a_kind.played);
    }
    if hands.straight_flush.played > 0 {
        println!("  Straight Flush: Lv.{} ({} chips, {}x mult) - Played {} times", 
                 hands.straight_flush.hand.level, hands.straight_flush.hand.chips, 
                 hands.straight_flush.hand.mult, hands.straight_flush.played);
    }
    
    if let Some(five_kind) = &hands.five_of_a_kind {
        if five_kind.played > 0 {
            println!("  Five of a Kind: Lv.{} ({} chips, {}x mult) - Played {} times", 
                     five_kind.hand.level, five_kind.hand.chips, 
                     five_kind.hand.mult, five_kind.played);
        }
    }
    if let Some(flush_house) = &hands.flush_house {
        if flush_house.played > 0 {
            println!("  Flush House: Lv.{} ({} chips, {}x mult) - Played {} times", 
                     flush_house.hand.level, flush_house.hand.chips, 
                     flush_house.hand.mult, flush_house.played);
        }
    }
    if let Some(flush_fives) = &hands.flush_fives {
        if flush_fives.played > 0 {
            println!("  Flush Fives: Lv.{} ({} chips, {}x mult) - Played {} times", 
                     flush_fives.hand.level, flush_fives.hand.chips, 
                     flush_fives.hand.mult, flush_fives.played);
        }
    }
    
    println!("\nBlinds Progress:");
    println!("  Small: {:?}", run_info.blinds.small);
    println!("  Big: {:?}", run_info.blinds.big);
    println!("  Boss: {:?}", run_info.blinds.boss);
}

fn print_hud<'a, T: Hud<'a>>(hud: &T) {
    println!("\n--- HUD INFO ---");
    println!("Hands: {} | Discards: {} | Money: ${}", hud.hands(), hud.discards(), hud.money());
    println!("Round: {} | Ante: {}", hud.round(), hud.ante());
    
    if !hud.jokers().is_empty() {
        println!("Jokers: {:?}", hud.jokers());
    }
    if !hud.consumables().is_empty() {
        println!("Consumables: {:?}", hud.consumables());
    }
    if !hud.tags().is_empty() {
        println!("Tags: {:?}", hud.tags());
    }
    print_run_info(hud.run_info());
    println!("----------------");
}

async fn handle_hud_management<'a, T: Hud<'a>>(mut screen: T) -> Result<T, Box<dyn std::error::Error>> {
    loop {
        display_hud_menu();
        print_hud(&screen);
        
        let action = get_string_input("Enter action:");
        match action.trim().to_lowercase().as_str() {
            "jokers" => {
                screen = handle_joker_management(screen).await?;
            },
            "consumables" => {
                screen = handle_consumable_management(screen).await?;
            },
            "tags" => {
                println!("\n--- TAGS ---");
                for (i, tag) in screen.tags().iter().enumerate() {
                    println!("  {}: {:?}", i, tag);
                }
                if screen.tags().is_empty() {
                    println!("  No tags available");
                }
                println!("Press Enter to continue");
                let _ = get_string_input("");
            },
            "back" => return Ok(screen),
            _ => println!("Invalid action. Use 'jokers', 'consumables', 'tags', or 'back'."),
        }
    }
}

async fn handle_joker_management<'a, T: Hud<'a>>(mut screen: T) -> Result<T, Box<dyn std::error::Error>> {
    loop {
        display_joker_menu();
        println!("\n--- JOKERS ---");
        let jokers = screen.jokers().to_vec();
        for (i, joker) in jokers.iter().enumerate() {
            println!("  {}: {:?}", i, joker);
        }
        if jokers.is_empty() {
            println!("  No jokers available");
        }
        
        let input = get_string_input("Enter action:");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        screen = match parts.get(0).map(|s| s.to_lowercase()).as_deref() {
            Some("move") => {
                if parts.len() >= 3 {
                    if let (Ok(from), Ok(to)) = (parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
                        match screen.move_joker(from, to).await {
                            Ok(updated_screen) => {
                                println!("Joker moved successfully");
                                updated_screen
                            },
                            Err(e) => {
                                error!("Failed to move joker: {}", e);
                                return Err(e.into());
                            },
                        }
                    } else {
                        println!("Invalid indices. Use numbers for positions.");
                        screen
                    }
                } else {
                    println!("Usage: move <from> <to> (e.g., 'move 0 1')");
                    screen
                }
            },
            Some("sell") => {
                if parts.len() >= 2 {
                    if let Ok(index) = parts[1].parse::<u32>() {
                        match screen.sell_joker(index).await {
                            Ok(updated_screen) => {
                                println!("Joker sold successfully");
                                updated_screen
                            },
                            Err(e) => {
                                error!("Failed to sell joker: {}", e);
                                return Err(e.into());
                            },
                        }
                    } else {
                        println!("Invalid index. Use a number.");
                        screen
                    }
                } else {
                    println!("Usage: sell <index> (e.g., 'sell 0')");
                    screen
                }
            },
            Some("back") => return Ok(screen),
            _ => {
                println!("Invalid action. Use 'move <from> <to>', 'sell <index>', or 'back'.");
                screen
            },
        };
    }
}

async fn handle_consumable_management<'a, T: Hud<'a>>(mut screen: T) -> Result<T, Box<dyn std::error::Error>> {
    loop {
        display_consumable_menu();
        println!("\n--- CONSUMABLES ---");
        let consumables = screen.consumables().to_vec();
        for (i, consumable) in consumables.iter().enumerate() {
            println!("  {}: {:?}", i, consumable);
        }
        if consumables.is_empty() {
            println!("  No consumables available");
        }
        
        let input = get_string_input("Enter action:");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        screen = match parts.get(0).map(|s| s.to_lowercase()).as_deref() {
            Some("use") => {
                if parts.len() >= 2 {
                    if let Ok(index) = parts[1].parse::<u32>() {
                        match screen.use_consumable(index).await {
                            Ok(updated_screen) => {
                                println!("Consumable used successfully");
                                updated_screen
                            },
                            Err(e) => {
                                error!("Failed to use consumable: {}", e);
                                return Err(e.into());
                            },
                        }
                    } else {
                        println!("Invalid index. Use a number.");
                        screen
                    }
                } else {
                    println!("Usage: use <index> (e.g., 'use 0')");
                    screen
                }
            },
            Some("move") => {
                if parts.len() >= 3 {
                    if let (Ok(from), Ok(to)) = (parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
                        match screen.move_consumable(from, to).await {
                            Ok(updated_screen) => {
                                println!("Consumable moved successfully");
                                updated_screen
                            },
                            Err(e) => {
                                error!("Failed to move consumable: {}", e);
                                return Err(e.into());
                            },
                        }
                    } else {
                        println!("Invalid indices. Use numbers for positions.");
                        screen
                    }
                } else {
                    println!("Usage: move <from> <to> (e.g., 'move 0 1')");
                    screen
                }
            },
            Some("sell") => {
                if parts.len() >= 2 {
                    if let Ok(index) = parts[1].parse::<u32>() {
                        match screen.sell_consumable(index).await {
                            Ok(updated_screen) => {
                                println!("Consumable sold successfully");
                                updated_screen
                            },
                            Err(e) => {
                                error!("Failed to sell consumable: {}", e);
                                return Err(e.into());
                            },
                        }
                    } else {
                        println!("Invalid index. Use a number.");
                        screen
                    }
                } else {
                    println!("Usage: sell <index> (e.g., 'sell 0')");
                    screen
                }
            },
            Some("back") => return Ok(screen),
            _ => {
                println!("Invalid action. Use 'use <index>', 'move <from> <to>', 'sell <index>', or 'back'.");
                screen
            },
        };
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
        info!("Starting game session");
        loop {
            // Check current screen in Game
            match balatro.screen().await {
                Ok(screen) => match screen {
                    CurrentScreen::Menu(menu) => {
                        display_menu();
                        if let Some(saved) = menu.saved_run() {
                            println!("Saved run available: Deck {:?}, Stake {:?}, Round {}, Ante {}", 
                                   saved.deck, saved.stake, saved.round, saved.ante);
                        }
                        
                        let action = get_string_input("Enter action:");
                        match action.trim().to_lowercase().as_str() {
                            "new" => {
                                let deck: Deck = get_input("Select Deck (Red/Blue/Yellow/Green/Black/Magic/Nebula/Ghost/Abandoned/Checkered/Zodiac/Painted/Anaglyph/Plasma/Erratic):");
                                let stake: Stake = get_input("Select Stake (White/Red/Green/Blue/Black/Purple/Orange/Gold):");
                                match menu.new_run(deck, stake, None).await {
                                    Ok(_) => {},
                                    Err(e) => error!("Failed to start new run: {}", e),
                                }
                            },
                            "resume" => {
                                if menu.saved_run().is_some() {
                                    let deck: Deck = get_input("Select Deck:");
                                    let stake: Stake = get_input("Select Stake:");
                                    match menu.resume_run(deck, stake, None).await {
                                        Ok(_) => {},
                                        Err(e) => error!("Failed to resume run: {}", e),
                                    }
                                } else {
                                    println!("No saved run available");
                                }
                            },
                            _ => println!("Invalid action. Use 'new' or 'resume'."),
                        }
                    }
                    CurrentScreen::SelectBlind(blinds) => {
                        display_blinds_menu();
                        print_hud(&blinds);
                        println!("\nBlind Options:");
                        println!("Small: {:?}", blinds.small());
                        println!("Big: {:?}", blinds.big());
                        println!("Boss: {:?}", blinds.boss());
                        
                        let action = get_string_input("Enter action:");
                        match action.trim().to_lowercase().as_str() {
                            "select" => {
                                match blinds.select().await {
                                    Ok(_) => println!("Blind selected successfully"),
                                    Err(e) => error!("Failed to select blind: {}", e),
                                }
                            },
                            "skip" => {
                                match blinds.skip().await {
                                    Ok(_) => println!("Blind skipped successfully"),
                                    Err(e) => error!("Failed to skip blind: {}", e),
                                }
                            },
                            "hud" => {
                                match handle_hud_management(blinds).await {
                                    Ok(_updated_blinds) => {
                                        println!("Returned from HUD management");
                                    },
                                    Err(e) => error!("HUD management error: {}", e),
                                }
                            },
                            _ => println!("Invalid action. Use 'select', 'skip', or 'hud'."),
                        }
                    }
                    CurrentScreen::Play(play) => {
                        display_play_menu();
                        println!("\nCurrent State:");
                        println!("Hand ({} cards):", play.hand().len());
                        for (i, hand_card) in play.hand().iter().enumerate() {
                            if let Some(card) = &hand_card.card {
                                let selection = if hand_card.selected { "[SELECTED]" } else { "         " };
                                
                                // Format the basic card
                                let rank_str = format!("{:?}", card.rank);
                                let suit_str = format!("{:?}", card.suit);
                                let mut card_parts = vec![format!("{} of {}", rank_str, suit_str)];
                                
                                // Add enhancement if present
                                if let Some(enhancement) = &card.enhancement {
                                    card_parts.push(format!("{:?}", enhancement));
                                }
                                
                                // Add edition if present
                                if let Some(edition) = &card.edition {
                                    card_parts.push(format!("{:?}", edition));
                                }
                                
                                // Add seal if present
                                if let Some(seal) = &card.seal {
                                    card_parts.push(format!("{:?} Seal", seal));
                                }
                                
                                // Combine all parts
                                let formatted_card = if card_parts.len() > 1 {
                                    format!("{} ({})", card_parts[0], card_parts[1..].join(", "))
                                } else {
                                    card_parts[0].clone()
                                };
                                
                                println!("  {}: {} {}", i, selection, formatted_card);
                            } else {
                                let selection = if hand_card.selected { "[SELECTED]" } else { "         " };
                                println!("  {}: {} [Empty Card Slot]", i, selection);
                            }
                        }
                        println!("Blind: {:?}", play.blind());
                        println!("Score: {}", play.score());
                        if let Some(poker_hand) = play.poker_hand() {
                            println!("Poker Hand: {:?}", poker_hand);
                        }
                        print_hud(&play);
                        
                        let input = get_string_input("Enter action:");
                        let parts: Vec<&str> = input.trim().split_whitespace().collect();
                        
                        match parts.get(0).map(|s| s.to_lowercase()).as_deref() {
                            Some("select") => {
                                if parts.len() > 1 {
                                    let indices: Result<Vec<u32>, _> = parts[1..].iter()
                                        .map(|s| s.parse())
                                        .collect();
                                    match indices {
                                        Ok(indices) => {
                                            match play.click(&indices).await {
                                                Ok(_) => println!("Cards selected successfully"),
                                                Err(e) => error!("Failed to select cards: {}", e),
                                            }
                                        },
                                        Err(_) => println!("Invalid card indices. Use numbers separated by spaces."),
                                    }
                                } else {
                                    println!("Please specify card indices (e.g., 'select 0 1 2')");
                                }
                            },
                            Some("play") => {
                                match play.play().await {
                                    Ok(PlayResult::Again(_)) => {
                                        println!("Continue playing...");
                                    },
                                    Ok(PlayResult::RoundOver(overview)) => {
                                        println!("\n=== ROUND COMPLETE ===");
                                        print_hud(&overview);
                                        println!("Total Earned: ${}", overview.total_earned());
                                        println!("Earnings Breakdown: {:?}", overview.earnings());
                                        display_overview_menu();
                                        let _ = get_string_input("Press Enter to continue to shop");
                                        match overview.cash_out().await {
                                            Ok(_) => println!("Proceeding to shop..."),
                                            Err(e) => error!("Failed to proceed to shop: {}", e),
                                        }
                                    },
                                    Ok(PlayResult::GameOver(game_over)) => {
                                        println!("\n=== GAME OVER ===");
                                        println!("Outcome: {:?}", game_over.outcome());
                                        if let Some(best) = game_over.best_hand() {
                                            println!("Best Hand: {}", best);
                                        }
                                        println!("Most Played Hand: {:?}", game_over.most_played_hand());
                                        println!("Seed: {:?}", game_over.seed());
                                        display_game_over_menu();
                                        let _ = get_string_input("Press Enter to return to menu");
                                        game_over.menu();
                                        return;
                                    },
                                    Err(e) => error!("Failed to play: {}", e),
                                }
                            },
                            Some("discard") => {
                                match play.discard().await {
                                    Ok(DiscardResult::Again(_)) => {
                                        println!("Continue with discarding...");
                                    },
                                    Ok(DiscardResult::GameOver(game_over)) => {
                                        println!("\n=== GAME OVER ===");
                                        println!("Outcome: {:?}", game_over.outcome());
                                        display_game_over_menu();
                                        let _ = get_string_input("Press Enter to return to menu");
                                        game_over.menu();
                                        return;
                                    },
                                    Err(e) => error!("Failed to discard: {}", e),
                                }
                            },
                            Some("hud") => {
                                match handle_hud_management(play).await {
                                    Ok(_updated_play) => {
                                        println!("Returned from HUD management");
                                    },
                                    Err(e) => error!("HUD management error: {}", e),
                                }
                            },
                            _ => println!("Invalid action. Use 'select <indices>', 'play', 'discard', or 'hud'."),
                        }
                    }
                    CurrentScreen::Shop(shop) => {
                        display_shop_menu();
                        print_hud(&shop);
                        println!("\nShop Contents:");
                        println!("Main Items: {:?}", shop.main_cards());
                        println!("Vouchers: {:?}", shop.vouchers());
                        println!("Boosters: {:?}", shop.boosters());
                        
                        let input = get_string_input("Enter action:");
                        let parts: Vec<&str> = input.trim().split_whitespace().collect();
                        
                        match parts.get(0).map(|s| s.to_lowercase()).as_deref() {
                            Some("buy") => {
                                if parts.len() >= 3 {
                                    let item_type = parts[1].to_lowercase();
                                    if let Ok(index) = parts[2].parse::<u8>() {
                                        match item_type.as_str() {
                                            "main" => {
                                                match shop.buy_main(index).await {
                                                    Ok(_) => println!("Item purchased successfully"),
                                                    Err(e) => error!("Failed to buy item: {}", e),
                                                }
                                            },
                                            "voucher" => {
                                                match shop.buy_voucher(index).await {
                                                    Ok(_) => println!("Voucher purchased successfully"),
                                                    Err(e) => error!("Failed to buy voucher: {}", e),
                                                }
                                            },
                                            "booster" => {
                                                match shop.buy_booster(index).await {
                                                    Ok(_) => println!("Booster purchased successfully"),
                                                    Err(e) => error!("Failed to buy booster: {}", e),
                                                }
                                            },
                                            _ => println!("Invalid item type. Use 'main', 'voucher', or 'booster'."),
                                        }
                                    } else {
                                        println!("Invalid index. Use a number.");
                                    }
                                } else {
                                    println!("Usage: buy <type> <index> (e.g., 'buy main 0')");
                                }
                            },
                            Some("reroll") => {
                                match shop.reroll().await {
                                    Ok(_) => println!("Shop rerolled successfully"),
                                    Err(e) => error!("Failed to reroll shop: {}", e),
                                }
                            },
                            Some("leave") => {
                                match shop.leave().await {
                                    Ok(_) => println!("Left shop successfully"),
                                    Err(e) => error!("Failed to leave shop: {}", e),
                                }
                            },
                            Some("hud") => {
                                match handle_hud_management(shop).await {
                                    Ok(_updated_shop) => {
                                        println!("Returned from HUD management");
                                    },
                                    Err(e) => error!("HUD management error: {}", e),
                                }
                            },
                            _ => println!("Invalid action. Use 'buy <type> <index>', 'reroll', 'leave', or 'hud'."),
                        }
                    }
                    CurrentScreen::ShopOpen(pack) => {
                        display_pack_menu();
                        
                        match pack {
                            OpenPack::Arcana(pack) => {
                                println!("\nArcana Pack:");
                                println!("Options: {:?}", pack.options());
                                let hand = pack.hand().await;
                                println!("Hand ({} cards):", hand.len());
                                for (i, card) in hand.iter().enumerate() {
                                    let selection = if card.selected { "[SELECTED]" } else { "         " };
                                    println!("  {}: {} {:?} of {:?}", i, selection, card.card.rank, card.card.suit);
                                    if let Some(enhancement) = &card.card.enhancement {
                                        println!("      Enhancement: {:?}", enhancement);
                                    }
                                    if let Some(edition) = &card.card.edition {
                                        println!("      Edition: {:?}", edition);
                                    }
                                    if let Some(seal) = &card.card.seal {
                                        println!("      Seal: {:?}", seal);
                                    }
                                }
                                println!("Selections left: {:?}", pack.selections_left());
                                
                                let input = get_string_input("Enter action:");
                                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                                
                                match parts.get(0).map(|s| s.to_lowercase()).as_deref() {
                                    Some("select") => {
                                        if parts.len() > 1 {
                                            if let Ok(index) = parts[1].parse::<u32>() {
                                                match pack.select(index).await {
                                                    Ok(_) => println!("Option selected successfully"),
                                                    Err(e) => error!("Failed to select option: {}", e),
                                                }
                                            } else {
                                                println!("Invalid index. Use a number.");
                                            }
                                        } else {
                                            println!("Please specify option index (e.g., 'select 0')");
                                        }
                                    },
                                    Some("click") => {
                                        if parts.len() > 1 {
                                            let indices: Result<Vec<u32>, _> = parts[1..].iter()
                                                .map(|s| s.parse())
                                                .collect();
                                            match indices {
                                                Ok(indices) => {
                                                    match pack.click(&indices).await {
                                                        Ok(_) => println!("Cards clicked successfully"),
                                                        Err(e) => error!("Failed to click cards: {}", e),
                                                    }
                                                },
                                                Err(_) => println!("Invalid indices"),
                                            }
                                        } else {
                                            println!("Please specify card indices (e.g., 'click 0 1')");
                                        }
                                    },
                                    Some("skip") => {
                                        match pack.skip().await {
                                            Ok(_) => println!("Skipped selection"),
                                            Err(e) => error!("Failed to skip: {}", e),
                                        }
                                    },
                                    _ => println!("Invalid action. Use 'select <index>', 'click <indices>', or 'skip'."),
                                }
                            },
                            OpenPack::Buffoon(pack) => {
                                println!("\nBuffoon Pack:");
                                println!("Options: {:?}", pack.options());
                                println!("Selections left: {:?}", pack.selections_left());
                                
                                let input = get_string_input("Enter action:");
                                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                                
                                match parts.get(0).map(|s| s.to_lowercase()).as_deref() {
                                    Some("select") => {
                                        if parts.len() > 1 {
                                            if let Ok(index) = parts[1].parse::<u32>() {
                                                match pack.select(index).await {
                                                    Ok(_) => println!("Joker selected successfully"),
                                                    Err(e) => error!("Failed to select joker: {}", e),
                                                }
                                            } else {
                                                println!("Invalid index. Use a number.");
                                            }
                                        } else {
                                            println!("Please specify joker index (e.g., 'select 0')");
                                        }
                                    },
                                    Some("skip") => {
                                        match pack.skip().await {
                                            Ok(_) => println!("Skipped selection"),
                                            Err(e) => error!("Failed to skip: {}", e),
                                        }
                                    },
                                    _ => println!("Invalid action. Use 'select <index>' or 'skip'."),
                                }
                            },
                            OpenPack::Celestial(pack) => {
                                println!("\nCelestial Pack:");
                                println!("Options: {:?}", pack.options());
                                println!("Selections left: {:?}", pack.selections_left());
                                
                                let input = get_string_input("Enter action:");
                                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                                
                                match parts.get(0).map(|s| s.to_lowercase()).as_deref() {
                                    Some("select") => {
                                        if parts.len() > 1 {
                                            if let Ok(index) = parts[1].parse::<u32>() {
                                                match pack.select(index).await {
                                                    Ok(_) => println!("Planet selected successfully"),
                                                    Err(e) => error!("Failed to select planet: {}", e),
                                                }
                                            } else {
                                                println!("Invalid index. Use a number.");
                                            }
                                        } else {
                                            println!("Please specify planet index (e.g., 'select 0')");
                                        }
                                    },
                                    Some("skip") => {
                                        match pack.skip().await {
                                            Ok(_) => println!("Skipped selection"),
                                            Err(e) => error!("Failed to skip: {}", e),
                                        }
                                    },
                                    _ => println!("Invalid action. Use 'select <index>' or 'skip'."),
                                }
                            },
                            OpenPack::Spectral(pack) => {
                                println!("\nSpectral Pack:");
                                println!("Options: {:?}", pack.options());
                                let hand = pack.hand().await;
                                println!("Hand ({} cards):", hand.len());
                                for (i, card) in hand.iter().enumerate() {
                                    let selection = if card.selected { "[SELECTED]" } else { "         " };
                                    println!("  {}: {} {:?} of {:?}", i, selection, card.card.rank, card.card.suit);
                                    if let Some(enhancement) = &card.card.enhancement {
                                        println!("      Enhancement: {:?}", enhancement);
                                    }
                                    if let Some(edition) = &card.card.edition {
                                        println!("      Edition: {:?}", edition);
                                    }
                                    if let Some(seal) = &card.card.seal {
                                        println!("      Seal: {:?}", seal);
                                    }
                                }
                                println!("Selections left: {:?}", pack.selections_left());
                                
                                let input = get_string_input("Enter action:");
                                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                                
                                match parts.get(0).map(|s| s.to_lowercase()).as_deref() {
                                    Some("select") => {
                                        if parts.len() > 1 {
                                            if let Ok(index) = parts[1].parse::<u32>() {
                                                match pack.select(index).await {
                                                    Ok(_) => println!("Option selected successfully"),
                                                    Err(e) => error!("Failed to select option: {}", e),
                                                }
                                            } else {
                                                println!("Invalid index. Use a number.");
                                            }
                                        } else {
                                            println!("Please specify option index (e.g., 'select 0')");
                                        }
                                    },
                                    Some("click") => {
                                        if parts.len() > 1 {
                                            let indices: Result<Vec<u32>, _> = parts[1..].iter()
                                                .map(|s| s.parse())
                                                .collect();
                                            match indices {
                                                Ok(indices) => {
                                                    match pack.click(&indices).await {
                                                        Ok(_) => println!("Cards clicked successfully"),
                                                        Err(e) => error!("Failed to click cards: {}", e),
                                                    }
                                                },
                                                Err(_) => println!("Invalid indices"),
                                            }
                                        } else {
                                            println!("Please specify card indices (e.g., 'click 0 1')");
                                        }
                                    },
                                    Some("skip") => {
                                        match pack.skip().await {
                                            Ok(_) => println!("Skipped selection"),
                                            Err(e) => error!("Failed to skip: {}", e),
                                        }
                                    },
                                    _ => println!("Invalid action. Use 'select <index>', 'click <indices>', or 'skip'."),
                                }
                            },
                            OpenPack::Standard(pack) => {
                                println!("\nStandard Pack:");
                                println!("Options: {:?}", pack.options());
                                println!("Selections left: {:?}", pack.selections_left());
                                
                                let input = get_string_input("Enter action:");
                                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                                
                                match parts.get(0).map(|s| s.to_lowercase()).as_deref() {
                                    Some("select") => {
                                        if parts.len() > 1 {
                                            if let Ok(index) = parts[1].parse::<u32>() {
                                                match pack.select(index).await {
                                                    Ok(_) => println!("Card selected successfully"),
                                                    Err(e) => error!("Failed to select card: {}", e),
                                                }
                                            } else {
                                                println!("Invalid index. Use a number.");
                                            }
                                        } else {
                                            println!("Please specify card index (e.g., 'select 0')");
                                        }
                                    },
                                    Some("skip") => {
                                        match pack.skip().await {
                                            Ok(_) => println!("Skipped selection"),
                                            Err(e) => error!("Failed to skip: {}", e),
                                        }
                                    },
                                    _ => println!("Invalid action. Use 'select <index>' or 'skip'."),
                                }
                            },
                        }
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
