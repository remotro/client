pub mod blinds;
pub mod menu;
pub mod hand;
pub mod deck;

use crate::net::Connection;

pub struct Balatro {
    connection: Connection,
}

impl Balatro {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Obtains the current state from the connected Balatro game.
    pub async fn screen(&mut self) -> Result<Screen, Error> {
        Ok(Screen::Menu(menu::Menu::new(&mut self.connection)))
    }
}

pub enum Screen<'a> {
    Menu(menu::Menu<'a>),
    Blinds(blinds::SelectBlind<'a>),
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
