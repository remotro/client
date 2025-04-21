use crate::net::Connection;
use crate::net::protocol;

pub struct Balatro {
    connection: Connection,
}

impl Balatro {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }
    pub async fn screen(&mut self) -> Result<Screen, Error> {
        Ok(Screen::Menu(MenuScreen { connection: &mut self.connection }))
    }
}

pub enum Screen<'a> {
    Menu(MenuScreen<'a>),
}

pub struct MenuScreen<'a> {
    connection: &'a mut Connection,
}

impl <'a> MenuScreen<'a> {
    pub async fn new_run(self) -> Result<Screen<'a>, Error> {
        let new_run = protocol::SetupRun {
            back: "b_yellow".to_string(),
            stake: 1,
            seed: None,
        };
        self.connection.req(new_run).await?.result?;
        Ok(Screen::Menu(self))
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

