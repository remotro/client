use log::{error, info};
use remotro::{
    balatro::{
        boosters::{BoosterCard, BoosterPackKind, Open, OpenWithHand, SelectResult},
        hud::Hud,
        menu::{Deck, Stake, Seed},
        play::{DiscardResult, PlayResult},
        shop::{BoosterPack, BoughtBooster, MainCard},
        Balatro,
        CurrentScreen
    },
    Remotro
};

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
    loop {
        let mut play_result = blind_select.select().await.unwrap().click(&[0, 1, 2]).await.unwrap().play().await.unwrap();
        let mut shop = as_variant!(play_result, PlayResult::RoundOver).cash_out().await.unwrap();
        for (i, booster) in shop.boosters().into_iter().enumerate() {
            if (booster.kind == BoosterPackKind::BuffoonNormal) {
                let mut buffoon = as_variant!(shop.buy_booster(i as u8).await.unwrap(), BoughtBooster::Buffoon);
                shop = as_variant!(buffoon.select(0).await.unwrap(), SelectResult::Done);
                break;
            }
        }
        blind_select = shop.leave().await.unwrap();
    }
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
    let mut balatro = remotro.accept().await.unwrap();
    quickrun(balatro).await;
}