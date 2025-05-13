pub mod blinds;
pub mod menu;
pub mod play;
pub mod deck;
pub mod shop;
pub mod util;
pub mod overview;

pub mod jokers;
pub mod consumables;
use crate::{
    net::Connection,
    balatro::{
        jokers::Joker,
        consumables::{
            Tarot,
            Planet,
            Spectral
        }
    }
};

pub struct Balatro {
    connection: Connection,
}

impl Balatro {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Obtains the current state from the connected Balatro game.
    pub async fn screen(&mut self) -> Result<Screen, Error> {
        let info = self.connection.request(protocol::GetScreen).await??;
        let screen = match info {
            protocol::ScreenInfo::Menu(_) => Screen::Menu(menu::Menu::new(&mut self.connection)),
            protocol::ScreenInfo::SelectBlind(blinds) => Screen::SelectBlind(blinds::SelectBlind::new(blinds, &mut self.connection)),
            protocol::ScreenInfo::Play(play) => Screen::Play(play::Play::new(play, &mut self.connection)),
            protocol::ScreenInfo::Shop(shop) => Screen::Shop(shop::Shop::new(shop, &mut self.connection)),
        };
        Ok(screen)
    }
}

pub enum Screen<'a> {
    Menu(menu::Menu<'a>),
    SelectBlind(blinds::SelectBlind<'a>),
    Play(play::Play<'a>),
    Shop(shop::Shop<'a>),
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

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};

    use crate::net::protocol::{Packet, Request, Response};

    use super::{blinds, play, shop};

    #[derive(Serialize, Deserialize)]
    pub struct GetScreen;

    impl Request for GetScreen {
        type Expect = Result<ScreenInfo, String>;
    }

    impl Packet for GetScreen {
        fn kind() -> String {
            "screen/get".to_string()
        }
    }

    #[derive(Serialize, Deserialize)]
    pub enum ScreenInfo {
        // Stupid workaround to make serde happy with { Menu = [] }
        Menu(Vec<()>),
        SelectBlind(blinds::protocol::BlindInfo),
        Play(play::protocol::PlayInfo),
        Shop(shop::protocol::ShopInfo),
    }

    impl Response for ScreenInfo {}

    impl Packet for ScreenInfo {
        fn kind() -> String {
            "screen/current".to_string()
        }
    }
}
