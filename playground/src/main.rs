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
#[tokio::main]
async fn main() {
    // Host a TCP socket
    let mut remotro = Remotro::host("127.0.0.1", 34143).await.unwrap();
    loop {
    let mut balatro = remotro.accept().await.unwrap();
    let screen = as_variant!(balatro.screen().await.unwrap(), CurrentScreen::Menu);
    let _ = screen.new_run(Deck::Red, Stake::White, None).await.unwrap().select().await;
    }
}
