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

#[tokio::main]
async fn main() {
    env_logger::init();

    // Host a TCP socket
    let mut remotro = Remotro::host("127.0.0.1", 34143).await.unwrap();
    let mut balatro = remotro.accept().await.unwrap();
}