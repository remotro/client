pub mod blinds;
pub mod menu;
pub mod play;
pub mod deck;
pub mod shop;
pub mod hud;
#[macro_use]
pub mod util;
pub mod overview;
pub mod jokers;
pub mod consumables;
pub mod boosters;
pub mod translations;

use serde::Deserialize;

use crate::balatro::blinds::{Boss, Tag};
use crate::balatro::boosters::BoosterPackKind;
use crate::balatro::consumables::{PlanetKind, SpectralKind, TarotKind};
use crate::balatro::deck::{CardEdition, Enhancement, Seal};
use crate::balatro::jokers::{JokerEdition, JokerKind};
use crate::balatro::shop::VoucherKind;
use crate::net::Connection;
use crate::net::protocol::Response;

pub struct Balatro {
    connection: Connection,
}

impl<'a> Balatro {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Obtains the current state from the connected Balatro game.
    pub async fn screen(&'a mut self) -> Result<CurrentScreen<'a>, Error> {
        let info = self.connection.request(protocol::GetScreen::<'a> { _r_marker: std::marker::PhantomData }).await??;
        let screen = match info {
            protocol::ScreenInfo::Menu(info) => CurrentScreen::Menu(menu::Menu::new(info, &mut self.connection)),
            protocol::ScreenInfo::SelectBlind(blinds) => CurrentScreen::SelectBlind(blinds::SelectBlind::new(blinds, &mut self.connection)),
            protocol::ScreenInfo::Play(play) => CurrentScreen::Play(play::Play::new(play, &mut self.connection)),
            protocol::ScreenInfo::RoundOverview(overview) => CurrentScreen::RoundOverview(overview::RoundOverview::new(overview, &mut self.connection)),
            protocol::ScreenInfo::Shop(shop) => CurrentScreen::Shop(shop::Shop::new(shop, &mut self.connection)),
            protocol::ScreenInfo::ShopOpen(pack) => match pack {
                shop::protocol::BoughtBooster::Arcana(info) => CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Arcana(boosters::OpenArcanaPack::new(info, &mut self.connection))),
                shop::protocol::BoughtBooster::Buffoon(info) => CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Buffoon(boosters::OpenBuffoonPack::new(info, &mut self.connection))),
                shop::protocol::BoughtBooster::Celestial(info) => CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Celestial(boosters::OpenCelestialPack::new(info, &mut self.connection))),
                shop::protocol::BoughtBooster::Spectral(info) => CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Spectral(boosters::OpenSpectralPack::new(info, &mut self.connection))),
                shop::protocol::BoughtBooster::Standard(info) => CurrentScreen::ShopOpen(boosters::OpenBoosterPack::Standard(boosters::OpenStandardPack::new(info, &mut self.connection))),
            },
            protocol::ScreenInfo::SkipOpen(pack) => match pack {
                blinds::protocol::SkippedBooster::Arcana(info) => CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Arcana(boosters::OpenArcanaPack::new(info, &mut self.connection))),
                blinds::protocol::SkippedBooster::Buffoon(info) => CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Buffoon(boosters::OpenBuffoonPack::new(info, &mut self.connection))),
                blinds::protocol::SkippedBooster::Celestial(info) => CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Celestial(boosters::OpenCelestialPack::new(info, &mut self.connection))),
                blinds::protocol::SkippedBooster::Spectral(info) => CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Spectral(boosters::OpenSpectralPack::new(info, &mut self.connection))),
                blinds::protocol::SkippedBooster::Standard(info) => CurrentScreen::SkipOpen(boosters::OpenBoosterPack::Standard(boosters::OpenStandardPack::new(info, &mut self.connection))),
            },
            protocol::ScreenInfo::GameOver(overview) => CurrentScreen::GameOver(overview::GameOverview::new(overview, &mut self.connection)),
        };
        Ok(screen)
    }
}

pub enum CurrentScreen<'a> {
    Menu(menu::Menu<'a>),
    SelectBlind(blinds::SelectBlind<'a>),
    Play(play::Play<'a>),
    RoundOverview(overview::RoundOverview<'a>), 
    Shop(shop::Shop<'a>),
    ShopOpen(boosters::OpenBoosterPack<'a, <shop::Shop<'a> as Screen<'a>>::Info>),
    SkipOpen(boosters::OpenBoosterPack<'a, blinds::protocol::SkipBlindResult<'a>>),
    GameOver(overview::GameOverview<'a>),
}

impl<'a> CurrentScreen<'a> {
    pub fn menu(self) -> menu::Menu<'a> {
        if let CurrentScreen::Menu(menu) = self {
            menu
        } else {
            panic!();
        }
    }

    pub fn play(self) -> play::Play<'a> {
        if let CurrentScreen::Play(play) = self {
            play
        } else {
            panic!();
        }
    }

    pub async fn collection(self) -> Result<Collection, crate::balatro::Error> {
        match self {
            CurrentScreen::Menu(menu) => menu.collection().await,
            CurrentScreen::SelectBlind(select_blind) => select_blind.collection().await,
            CurrentScreen::Play(play) => play.collection().await,
            CurrentScreen::RoundOverview(overview) => overview.collection().await,
            CurrentScreen::Shop(shop) => shop.collection().await,
            CurrentScreen::ShopOpen(open_pack) => open_pack.collection().await,
            CurrentScreen::SkipOpen(skip_pack) => skip_pack.collection().await,
            CurrentScreen::GameOver(game_over) => game_over.collection().await,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Net(crate::net::Error),
    Game(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

pub trait Screen<'a> {
    type Info: Response;
    fn name() -> String;
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self;
    async fn collection(self) -> Result<Collection, crate::balatro::Error>;
}

#[derive(Deserialize, Clone, Debug)]
pub struct Collection {
    pub jokers: Vec<JokerKind>,
    pub tarots: Vec<TarotKind>,
    pub spectrals: Vec<SpectralKind>,
    pub planets: Vec<PlanetKind>,
    pub vouchers: Vec<VoucherKind>,
    pub tags: Vec<Tag>,
    pub boss_blinds: Vec<Boss>,
    pub finisher_blinds: Vec<Boss>,
    pub booster_packs: Vec<BoosterPackKind>,
    pub enhancements: Vec<Enhancement>,
    pub card_editions: Vec<CardEdition>,
    pub joker_editions: Vec<JokerEdition>,
    pub seals: Vec<Seal>,
    pub blind_scaling: Vec<u64>
}

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};

    use crate::{balatro::{menu, overview, Collection}, net::protocol::{Packet, Request, Response}};

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

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct GetCollection;

    impl Request for GetCollection {
        type Expect = Result<CollectionInfo, String>;
    }

    impl Packet for GetCollection {
        fn kind() -> String {
            "collection/get".to_string()
        }
    }

    #[derive(Deserialize, Clone, Debug)]
    pub struct CollectionInfo {
        pub collection: Collection,
    }

    impl Packet for CollectionInfo {
        fn kind() -> String {
            "collection/info".to_string()
        }
    }

    impl Response for CollectionInfo {}
}
