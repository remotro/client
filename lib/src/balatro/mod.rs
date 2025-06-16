pub mod blinds;
pub mod menu;
pub mod play;
pub mod deck;
pub mod shop;
pub mod hud;
pub mod util;
pub mod overview;
pub mod jokers;
pub mod consumables;
pub mod boosters;

use crate::net::Connection;
use crate::net::protocol::Response;

pub struct Balatro<'a> {
    _r_marker: std::marker::PhantomData<&'a ()>,
    connection: Connection,
}

impl<'a> Balatro<'a> {
    pub fn new(connection: Connection) -> Self {
        Self { connection, _r_marker: std::marker::PhantomData }
    }

    /// Obtains the current state from the connected Balatro game.
    pub async fn screen(&mut self) -> Result<CurrentScreen, Error> {
        let info = self.connection.request(protocol::GetScreen::<'a> { _r_marker: std::marker::PhantomData }).await??;
        let screen = match info {
            protocol::ScreenInfo::Menu(info) => CurrentScreen::Menu(menu::Menu::new(&mut self.connection, info)),
            protocol::ScreenInfo::SelectBlind(blinds) => CurrentScreen::SelectBlind(blinds::SelectBlind::new(blinds, &mut self.connection)),
            protocol::ScreenInfo::Play(play) => CurrentScreen::Play(play::Play::new(play, &mut self.connection)),
            protocol::ScreenInfo::Shop(shop) => CurrentScreen::Shop(shop::Shop::new(shop, &mut self.connection)),
            protocol::ScreenInfo::OpenShopPack(pack) => match pack {
                protocol::OpenShopPackInfo::Arcana(info) => CurrentScreen::OpenShopPack(OpenPack::Arcana(boosters::OpenArcanaPack::new(info, &mut self.connection))),
                protocol::OpenShopPackInfo::Buffoon(info) => CurrentScreen::OpenShopPack(OpenPack::Buffoon(boosters::OpenBuffoonPack::new(info, &mut self.connection))),
                protocol::OpenShopPackInfo::Celestial(info) => CurrentScreen::OpenShopPack(OpenPack::Celestial(boosters::OpenSpectralPack::new(info, &mut self.connection))),
                protocol::OpenShopPackInfo::Spectral(info) => CurrentScreen::OpenShopPack(OpenPack::Spectral(boosters::OpenSpectralPack::new(info, &mut self.connection))),
                protocol::OpenShopPackInfo::Standard(info) => CurrentScreen::OpenShopPack(OpenPack::Standard(boosters::OpenStandardPack::new(info, &mut self.connection))),
            }
        };
        Ok(screen)
    }
}

pub enum CurrentScreen<'a> {
    Menu(menu::Menu<'a>),
    SelectBlind(blinds::SelectBlind<'a>),
    Play(play::Play<'a>),
    Shop(shop::Shop<'a>),
    OpenShopPack(OpenPack<'a, shop::Shop<'a>>)
}

pub enum OpenPack<'a, R: Screen<'a>> {
    Arcana(boosters::OpenArcanaPack<'a, R>),
    Buffoon(boosters::OpenBuffoonPack<'a, R>),
    Celestial(boosters::OpenSpectralPack<'a, R>),
    Spectral(boosters::OpenSpectralPack<'a, R>),
    Standard(boosters::OpenStandardPack<'a, R>)
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
    fn name() -> &'static str;
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self;
}

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};
    use crate::{
        balatro::{boosters, menu, Screen},
        net::protocol::{Packet, Request, Response}
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
        Shop(shop::protocol::ShopInfo),
        OpenShopPack(OpenShopPackInfo<'a>),
    }

    #[derive(Deserialize)]
    pub enum OpenShopPackInfo<'a> {
        Arcana(<boosters::OpenArcanaPack<'a, shop::Shop<'a>> as Screen<'a>>::Info),
        Buffoon(<boosters::OpenBuffoonPack<'a, shop::Shop<'a>> as Screen<'a>>::Info),
        Celestial(<boosters::OpenSpectralPack<'a, shop::Shop<'a>> as Screen<'a>>::Info),
        Spectral(<boosters::OpenSpectralPack<'a, shop::Shop<'a>> as Screen<'a>>::Info),
        Standard(<boosters::OpenStandardPack<'a, shop::Shop<'a>> as Screen<'a>>::Info),
    }

    impl<'a> Response for ScreenInfo<'a> {}

    impl<'a> Packet for ScreenInfo<'a> {
        fn kind() -> String {
            "screen/current".to_string()
        }
    }


}
