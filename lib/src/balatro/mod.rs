pub mod blinds;
pub mod deck;
#[doc(hidden)]
pub mod hud;
pub mod menu;
pub mod play;
pub mod shop;
#[macro_use]
#[doc(hidden)]
pub mod util;
pub mod boosters;
pub mod consumables;
pub mod jokers;
pub mod overview;

use crate::net::Connection;
use crate::net::protocol::Response;

pub struct Balatro {
    connection: Connection,
}

impl<'a> Balatro {
    #[doc(hidden)]
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Obtains the current state from the connected Balatro game.
    pub async fn screen(&'a mut self) -> Result<CurrentScreen<'a>, Error> {
        let info = self
            .connection
            .request(protocol::GetScreen::<'a> {
                _r_marker: std::marker::PhantomData,
            })
            .await??;
        let screen = match info {
            protocol::ScreenInfo::Menu(info) => {
                CurrentScreen::Menu(menu::Menu::new(&mut self.connection, info))
            }
            protocol::ScreenInfo::SelectBlind(blinds) => {
                CurrentScreen::SelectBlind(blinds::SelectBlind::new(blinds, &mut self.connection))
            }
            protocol::ScreenInfo::Play(play) => {
                CurrentScreen::Play(play::Play::new(play, &mut self.connection))
            }
            protocol::ScreenInfo::RoundOverview(overview) => CurrentScreen::RoundOverview(
                overview::RoundOverview::new(overview, &mut self.connection),
            ),
            protocol::ScreenInfo::Shop(shop) => {
                CurrentScreen::Shop(shop::Shop::new(shop, &mut self.connection))
            }
            protocol::ScreenInfo::ShopOpen(pack) => match pack {
                shop::protocol::BoughtBooster::Arcana(info) => {
                    CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Arcana(
                        boosters::OpenArcanaPack::new(info, &mut self.connection),
                    ))
                }
                shop::protocol::BoughtBooster::Buffoon(info) => {
                    CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Buffoon(
                        boosters::OpenBuffoonPack::new(info, &mut self.connection),
                    ))
                }
                shop::protocol::BoughtBooster::Celestial(info) => {
                    CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Celestial(
                        boosters::OpenCelestialPack::new(info, &mut self.connection),
                    ))
                }
                shop::protocol::BoughtBooster::Spectral(info) => {
                    CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Spectral(
                        boosters::OpenSpectralPack::new(info, &mut self.connection),
                    ))
                }
                shop::protocol::BoughtBooster::Standard(info) => {
                    CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Standard(
                        boosters::OpenStandardPack::new(info, &mut self.connection),
                    ))
                }
            },
            protocol::ScreenInfo::SkipOpen(pack) => match pack {
                blinds::protocol::SkippedBooster::Arcana(info) => {
                    CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Arcana(
                        boosters::OpenArcanaPack::new(info, &mut self.connection),
                    ))
                }
                blinds::protocol::SkippedBooster::Buffoon(info) => {
                    CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Buffoon(
                        boosters::OpenBuffoonPack::new(info, &mut self.connection),
                    ))
                }
                blinds::protocol::SkippedBooster::Celestial(info) => {
                    CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Celestial(
                        boosters::OpenCelestialPack::new(info, &mut self.connection),
                    ))
                }
                blinds::protocol::SkippedBooster::Spectral(info) => {
                    CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Spectral(
                        boosters::OpenSpectralPack::new(info, &mut self.connection),
                    ))
                }
                blinds::protocol::SkippedBooster::Standard(info) => {
                    CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Standard(
                        boosters::OpenStandardPack::new(info, &mut self.connection),
                    ))
                }
            },
            protocol::ScreenInfo::GameOver(overview) => {
                CurrentScreen::GameOver(overview::GameOverview::new(overview, &mut self.connection))
            }
        };
        Ok(screen)
    }
}

pub enum CurrentScreen<'a> {
    /// The Main menu / Title screen, where you can start a new run or load an old run
    Menu(menu::Menu<'a>),
    /// The Blind selection, containing the 3 blinds in the ante
    SelectBlind(blinds::SelectBlind<'a>),
    /// The playing section when playing against each blind
    Play(play::Play<'a>),
    /// The summary of earnings at the end of the blind
    RoundOverview(overview::RoundOverview<'a>),
    /// The shop between blinds
    Shop(shop::Shop<'a>),
    /// Booster packs opened from the Shop
    ShopOpen(boosters::OpenBoosterPack<'a, <shop::Shop<'a> as Screen<'a>>::Info>),
    /// Booster packs opened from skip tags
    SkipOpen(boosters::OpenBoosterPack<'a, blinds::protocol::SkipBlindResult<'a>>),
    /// Game Lost (Currently not working from the mod side)
    GameOver(overview::GameOverview<'a>),
}

#[doc(hidden)]
#[derive(Debug)]
pub enum Error {
    Net(crate::net::Error),
    Game(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<crate::net::Error> for Error {
    fn from(err: crate::net::Error) -> Self {
        Error::Net(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Game(err)
    }
}

#[doc(hidden)]
pub trait Screen<'a> {
    type Info: Response;
    fn name() -> String;
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self;
}

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};

    use crate::{
        balatro::{menu, overview},
        net::protocol::{Packet, Request, Response},
    };

    use super::{blinds, play, shop};

    #[derive(Serialize, Deserialize)]
    pub struct GetScreen<'a> {
        pub _r_marker: std::marker::PhantomData<&'a ()>,
    }

    impl<'a> Request for GetScreen<'a> {
        type Expect = Result<ScreenInfo<'a>, String>;
    }

    impl<'a> Packet for GetScreen<'a> {
        fn kind() -> String {
            "screen/get".to_string()
        }
    }

    #[derive(Deserialize)]
    pub enum ScreenInfo<'a> {
        Menu(menu::protocol::MenuInfo),
        SelectBlind(blinds::protocol::BlindInfo),
        Play(play::protocol::PlayInfo),
        RoundOverview(overview::protocol::RoundOverviewInfo),
        Shop(shop::protocol::ShopInfo),
        ShopOpen(shop::protocol::BoughtBooster<'a>),
        SkipOpen(blinds::protocol::SkippedBooster<'a>),
        GameOver(overview::protocol::GameOverviewInfo),
    }

    impl<'a> Response for ScreenInfo<'a> {}

    impl<'a> Packet for ScreenInfo<'a> {
        fn kind() -> String {
            "screen/current".to_string()
        }
    }
}
