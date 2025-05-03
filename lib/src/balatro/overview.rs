use crate::net::Connection;

pub struct RoundOverview<'a> {
    connection: &'a mut Connection,
}

impl<'a> RoundOverview<'a> {
    pub(crate) fn new(connection: &'a mut Connection) -> Self {
        Self { connection }
    }
}

pub struct GameOverview<'a> {
    connection: &'a mut Connection,
}

impl<'a> GameOverview<'a> {
    pub(crate) fn new(connection: &'a mut Connection) -> Self {
        Self { connection }
    }
}
