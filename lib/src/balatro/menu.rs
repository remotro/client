use crate::{
    balatro::{
        CurrentScreen, Screen, blinds, blinds::SelectBlind, boosters, overview, play::Play, shop,
        shop::Shop,
    },
    balatro_enum,
    net::Connection,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Display;
use std::str::FromStr;

pub struct Menu<'a> {
    connection: &'a mut Connection,
    info: protocol::MenuInfo,
}

impl<'a> Menu<'a> {
    pub(crate) fn new(connection: &'a mut Connection, info: protocol::MenuInfo) -> Self {
        Self { connection, info }
    }

    pub fn saved_run(&self) -> Option<&SavedRun> {
        self.info.saved_run.as_ref()
    }

    pub async fn new_run(
        self,
        deck: Deck,
        stake: Stake,
        seed: Option<Seed>,
    ) -> Result<SelectBlind<'a>, super::Error> {
        let new_run = protocol::StartRun { deck, stake, seed };
        let blinds = self.connection.request(new_run).await??;
        Ok(SelectBlind::new(blinds, self.connection))
    }

    pub async fn continue_run(self) -> Result<CurrentScreen<'a>, super::Error> {
        let continue_run = protocol::ContinueRun::<'a> {
            _r_marker: std::marker::PhantomData,
        };
        let screen: crate::balatro::protocol::ScreenInfo<'a> =
            self.connection.request(continue_run).await??;
        match screen {
            crate::balatro::protocol::ScreenInfo::SelectBlind(blinds) => Ok(
                CurrentScreen::SelectBlind(SelectBlind::new(blinds, self.connection)),
            ),
            crate::balatro::protocol::ScreenInfo::Play(play) => {
                Ok(CurrentScreen::Play(Play::new(play, self.connection)))
            }
            crate::balatro::protocol::ScreenInfo::RoundOverview(overview) => {
                Ok(CurrentScreen::RoundOverview(overview::RoundOverview::new(
                    overview,
                    self.connection,
                )))
            }
            crate::balatro::protocol::ScreenInfo::Shop(shop) => {
                Ok(CurrentScreen::Shop(Shop::new(shop, self.connection)))
            }
            crate::balatro::protocol::ScreenInfo::Menu(info) => {
                Ok(CurrentScreen::Menu(Menu::new(self.connection, info)))
            }
            crate::balatro::protocol::ScreenInfo::ShopOpen(pack) => match pack {
                shop::protocol::BoughtBooster::Arcana(info) => {
                    Ok(CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Arcana(
                        boosters::OpenArcanaPack::new(info, self.connection),
                    )))
                }
                shop::protocol::BoughtBooster::Buffoon(info) => {
                    Ok(CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Buffoon(
                        boosters::OpenBuffoonPack::new(info, self.connection),
                    )))
                }
                shop::protocol::BoughtBooster::Celestial(info) => Ok(CurrentScreen::ShopOpen(
                    boosters::OpenBoosterPack::Celestial(boosters::OpenCelestialPack::new(
                        info,
                        self.connection,
                    )),
                )),
                shop::protocol::BoughtBooster::Spectral(info) => Ok(CurrentScreen::ShopOpen(
                    boosters::OpenBoosterPack::Spectral(boosters::OpenSpectralPack::new(
                        info,
                        self.connection,
                    )),
                )),
                shop::protocol::BoughtBooster::Standard(info) => Ok(CurrentScreen::ShopOpen(
                    boosters::OpenBoosterPack::Standard(boosters::OpenStandardPack::new(
                        info,
                        self.connection,
                    )),
                )),
            },
            crate::balatro::protocol::ScreenInfo::SkipOpen(pack) => {
                match pack {
                    blinds::protocol::SkippedBooster::Arcana(info) => {
                        Ok(CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Arcana(
                            boosters::OpenArcanaPack::new(info, self.connection),
                        )))
                    }
                    blinds::protocol::SkippedBooster::Buffoon(info) => {
                        Ok(CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Buffoon(
                            boosters::OpenBuffoonPack::new(info, self.connection),
                        )))
                    }
                    blinds::protocol::SkippedBooster::Celestial(info) => Ok(
                        CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Celestial(
                            boosters::OpenCelestialPack::new(info, self.connection),
                        )),
                    ),
                    blinds::protocol::SkippedBooster::Spectral(info) => Ok(
                        CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Spectral(
                            boosters::OpenSpectralPack::new(info, self.connection),
                        )),
                    ),
                    blinds::protocol::SkippedBooster::Standard(info) => Ok(
                        CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Standard(
                            boosters::OpenStandardPack::new(info, self.connection),
                        )),
                    ),
                }
            }
            crate::balatro::protocol::ScreenInfo::GameOver(overview) => Ok(
                CurrentScreen::GameOver(overview::GameOverview::new(overview, self.connection)),
            ),
        }
    }
}

balatro_enum! {
    Deck {
        Red = "b_red",
        Blue = "b_blue",
        Yellow = "b_yellow",
        Green = "b_green",
        Black = "b_black",
        Magic = "b_magic",
        Nebula = "b_nebula",
        Ghost = "b_ghost",
        Abandoned = "b_abandoned",
        Checkered = "b_checkered",
        Zodiac = "b_zodiac",
        Painted = "b_painted",
        Anaglyph = "b_anaglyph",
        Plasma = "b_plasma",
        Erratic = "b_erratic",
    }
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
            _ => Err(format!(
                "Invalid deck. Valid options are: {}",
                [
                    "Red",
                    "Blue",
                    "Yellow",
                    "Green",
                    "Black",
                    "Magic",
                    "Nebula",
                    "Ghost",
                    "Abandoned",
                    "Checkered",
                    "Zodiac",
                    "Painted",
                    "Anaglyph",
                    "Plasma",
                    "Erratic"
                ]
                .join(", ")
            )),
        }
    }
}
impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Deck::Red => "Red",
            Deck::Blue => "Blue",
            Deck::Yellow => "Yellow",
            Deck::Green => "Green",
            Deck::Black => "Black",
            Deck::Magic => "Magic",
            Deck::Nebula => "Nebula",
            Deck::Ghost => "Ghost",
            Deck::Abandoned => "Abandoned",
            Deck::Checkered => "Checkered",
            Deck::Zodiac => "Zodiac",
            Deck::Painted => "Painted",
            Deck::Anaglyph => "Anaglyph",
            Deck::Plasma => "Plasma",
            Deck::Erratic => "Erratic",
        }
        .to_string();
        write!(f, "{}", str)
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
            _ => Err(format!(
                "Invalid stake. Valid options are: {}",
                [
                    "White", "Red", "Green", "Black", "Blue", "Purple", "Orange", "Gold"
                ]
                .join(", ")
            )),
        }
    }
}
impl Display for Stake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Stake::White => "White",
            Stake::Red => "Red",
            Stake::Green => "Green",
            Stake::Black => "Black",
            Stake::Blue => "Blue",
            Stake::Purple => "Purple",
            Stake::Orange => "Orange",
            Stake::Gold => "Gold",
        }
        .to_string();
        write!(f, "{}", str)
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
    pub money: u64,
}
impl Display for SavedRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} deck, {} stake, ante {} round {}, ${}",
            self.deck, self.stake, self.ante, self.round, self.money
        )
    }
}

pub(crate) mod protocol {
    use super::{Deck, SavedRun, Seed, Stake};
    use crate::{
        balatro::blinds::protocol::BlindInfo,
        net::protocol::{Packet, Request},
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct MenuInfo {
        pub saved_run: Option<SavedRun>,
    }

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
