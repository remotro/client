use crate::balatro::boosters;
use crate::balatro::overview;
use crate::balatro::play::Play;
use crate::balatro::shop;
use crate::balatro::shop::Shop;
use crate::balatro::blinds;
use crate::balatro::{blinds::SelectBlind, CurrentScreen, Screen};
use crate::net::Connection;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use serde_repr::{Deserialize_repr, Serialize_repr};

pub struct Menu<'a> {
    connection: &'a mut Connection,
    info: protocol::MenuInfo,
}

impl<'a> Menu<'a> {
    pub fn saved_run(&self) -> Option<&SavedRun> {
        self.info.saved_run.as_ref()
    }

    pub async fn new_run(
        self,
        deck: Deck,
        stake: Stake,
        seed: Option<Seed>,
    ) -> Result<SelectBlind<'a>, super::Error> {
        let new_run = protocol::StartRun {
            deck,
            stake,
            seed,
        };
        let blinds = self.connection.request(new_run).await??;
        Ok(SelectBlind::new(blinds, self.connection))
    }

    pub async fn continue_run(
        self
    ) -> Result<CurrentScreen<'a>, super::Error> {
        let continue_run = protocol::ContinueRun::<'a> {
            _r_marker: std::marker::PhantomData,
        };
        let screen: crate::balatro::protocol::ScreenInfo<'a> = self.connection.request(continue_run).await??;
        match screen {
            crate::balatro::protocol::ScreenInfo::SelectBlind(blinds) => Ok(CurrentScreen::SelectBlind(SelectBlind::new(blinds, self.connection))),
            crate::balatro::protocol::ScreenInfo::Play(play) => Ok(CurrentScreen::Play(Play::new(play, self.connection))),
            crate::balatro::protocol::ScreenInfo::RoundOverview(overview) => Ok(CurrentScreen::RoundOverview(overview::RoundOverview::new(overview, self.connection))),
            crate::balatro::protocol::ScreenInfo::Shop(shop) => Ok(CurrentScreen::Shop(Shop::new(shop, self.connection))),
            crate::balatro::protocol::ScreenInfo::Menu(info) => Ok(CurrentScreen::Menu(Menu::new(info, self.connection))),
            crate::balatro::protocol::ScreenInfo::ShopOpen(pack) => match pack {
                shop::protocol::BoughtBooster::Arcana(info) => Ok(CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Arcana(boosters::OpenArcanaPack::new(info, self.connection)))),
                shop::protocol::BoughtBooster::Buffoon(info) => Ok(CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Buffoon(boosters::OpenBuffoonPack::new(info, self.connection)))),
                shop::protocol::BoughtBooster::Celestial(info) => Ok(CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Celestial(boosters::OpenCelestialPack::new(info, self.connection)))),
                shop::protocol::BoughtBooster::Spectral(info) => Ok(CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Spectral(boosters::OpenSpectralPack::new(info, self.connection)))),
                shop::protocol::BoughtBooster::Standard(info) => Ok(CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Standard(boosters::OpenStandardPack::new(info, self.connection)))),
            },
            crate::balatro::protocol::ScreenInfo::SkipOpen(pack) => match pack {
                blinds::protocol::SkippedBooster::Arcana(info) => Ok(CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Arcana(boosters::OpenArcanaPack::new(info, self.connection)))),
                blinds::protocol::SkippedBooster::Buffoon(info) => Ok(CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Buffoon(boosters::OpenBuffoonPack::new(info, self.connection)))),
                blinds::protocol::SkippedBooster::Celestial(info) => Ok(CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Celestial(boosters::OpenCelestialPack::new(info, self.connection)))),
                blinds::protocol::SkippedBooster::Spectral(info) => Ok(CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Spectral(boosters::OpenSpectralPack::new(info, self.connection)))),
                blinds::protocol::SkippedBooster::Standard(info) => Ok(CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Standard(boosters::OpenStandardPack::new(info, self.connection)))),
            },
            crate::balatro::protocol::ScreenInfo::GameOver(overview) => Ok(CurrentScreen::GameOver(overview::GameOverview::new(overview, self.connection))),
        }
    }
}

impl<'a> Screen<'a> for Menu<'a> {
    type Info = protocol::MenuInfo;
    fn name() -> String {
        "menu".to_string()
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Deck {
    #[serde(rename = "b_red")]
    Red,
    #[serde(rename = "b_blue")]
    Blue,
    #[serde(rename = "b_yellow")]
    Yellow,
    #[serde(rename = "b_green")]
    Green,
    #[serde(rename = "b_black")]
    Black,
    #[serde(rename = "b_magic")]
    Magic,
    #[serde(rename = "b_nebula")]
    Nebula,
    #[serde(rename = "b_ghost")]
    Ghost,
    #[serde(rename = "b_abandoned")]
    Abandoned,
    #[serde(rename = "b_checkered")]
    Checkered,
    #[serde(rename = "b_zodiac")]
    Zodiac,
    #[serde(rename = "b_painted")]
    Painted,
    #[serde(rename = "b_anaglyph")]
    Anaglyph,
    #[serde(rename = "b_plasma")]
    Plasma,
    #[serde(rename = "b_erratic")]
    Erratic,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Stake {
    White = 1,
    Red = 2,
    Green = 3,
    Black = 4,
    Blue = 5,
    Purple = 6,
    Orange = 7,
    Gold = 8,
}

impl FromStr for Deck {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "red" => Ok(Deck::Red),
            "blue" => Ok(Deck::Blue),
            "yellow" => Ok(Deck::Yellow),
            "green" => Ok(Deck::Green),
            "black" => Ok(Deck::Black),
            "magic" => Ok(Deck::Magic),
            "nebula" => Ok(Deck::Nebula),
            "ghost" => Ok(Deck::Ghost),
            "abandoned" => Ok(Deck::Abandoned),
            "checkered" => Ok(Deck::Checkered),
            "zodiac" => Ok(Deck::Zodiac),
            "painted" => Ok(Deck::Painted),
            "anaglyph" => Ok(Deck::Anaglyph),
            "plasma" => Ok(Deck::Plasma),
            "erratic" => Ok(Deck::Erratic),
            _ => Err(format!("Invalid deck. Valid options are: {}", 
                ["Red", "Blue", "Yellow", "Green", "Black", "Magic", "Nebula", 
                 "Ghost", "Abandoned", "Checkered", "Zodiac", "Painted", 
                 "Anaglyph", "Plasma", "Erratic"].join(", ")))
        }
    }
}

impl FromStr for Stake {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "white" => Ok(Stake::White),
            "red" => Ok(Stake::Red),
            "green" => Ok(Stake::Green),
            "black" => Ok(Stake::Black),
            "blue" => Ok(Stake::Blue),
            "purple" => Ok(Stake::Purple),
            "orange" => Ok(Stake::Orange),
            "gold" => Ok(Stake::Gold),
            _ => Err(format!("Invalid stake. Valid options are: {}",
                ["White", "Red", "Green", "Black", "Blue", "Purple", 
                 "Orange", "Gold"].join(", ")))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Seed(String);

impl Seed {
    pub fn new(seed: String) -> Option<Self> {
        if seed.len() == 7 {
            Some(Self(seed))
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SavedRun {
    pub deck: Deck,
    pub stake: Stake,
    pub best_hand: u64,
    pub round: u64,
    pub ante: u64,
    pub money: u64
}

pub(crate) mod protocol {
    use super::{Deck, Seed, Stake, SavedRun};
    use crate::{
        balatro::blinds::protocol::BlindInfo,
        net::protocol::{Packet, Request, Response},
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct MenuInfo {
        pub saved_run: Option<SavedRun>,
    }

    impl Packet for MenuInfo {
        fn kind() -> String {
            "menu/info".to_string()
        }
    }

    impl Response for MenuInfo {}

    // Hide serialization impls here since they're specific to Balatro's
    // internals.

    #[derive(Serialize)]
    pub struct StartRun {
        pub deck: Deck,
        pub stake: Stake,
        pub seed: Option<Seed>,
    }

    impl Request for StartRun {
        type Expect = Result<BlindInfo, String>;
    }

    impl Packet for StartRun {
        fn kind() -> String {
            "main_menu/start_run".to_string()
        }
    }

    #[derive(Serialize)]
    pub struct ContinueRun<'a> {
        pub _r_marker: std::marker::PhantomData<&'a ()>,
    }
    
    impl<'a> Request for ContinueRun<'a> {
        type Expect = Result<crate::balatro::protocol::ScreenInfo<'a>, String>;
    }

    impl<'a> Packet for ContinueRun<'a> {
        fn kind() -> String {
            "main_menu/continue_run".to_string()
        }
    }
}
