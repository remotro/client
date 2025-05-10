use serde::{Deserialize, Serialize};

use crate::net::Connection;
use super::{deck::Card, overview::{GameOverview, RoundOverview}, Error, blinds::CurrentBlind};

pub struct Play<'a> {
    info: protocol::PlayInfo,
    connection: &'a mut Connection,
}

impl<'a> Play<'a> {
    pub(crate) fn new(info: protocol::PlayInfo, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }

    pub fn hand(&self) -> &[HandCard] {
        &self.info.hand
    }

    pub fn current_blind(&self) -> &CurrentBlind {
        &self.info.current_blind
    }

    pub async fn click(self, indices: &[u32]) -> Result<Self, Error> {
        let info = self.connection.request(protocol::PlayClick { indices: indices.to_vec() }).await??;
        Ok(Self::new(info, self.connection))
    }

    pub async fn play(self) -> Result<PlayResult<'a>, Error> {
        let info = self.connection.request(protocol::PlayPlay).await??;
        let result = match info {
            protocol::PlayResult::Again(info) => PlayResult::Again(Self::new(info, self.connection)),
            protocol::PlayResult::RoundOver(_) => PlayResult::RoundOver(RoundOverview::new(self.connection)),
            protocol::PlayResult::GameOver(_) => PlayResult::GameOver(GameOverview::new(self.connection)),
        };
        Ok(result)
    }

    pub async fn discard(self) -> Result<DiscardResult<'a>, Error> {
        let info = self.connection.request(protocol::PlayDiscard).await??;
        let result = match info {
            protocol::DiscardResult::Again(info) => DiscardResult::Again(Self::new(info, self.connection)),
            protocol::DiscardResult::GameOver(_) => DiscardResult::GameOver(GameOverview::new(self.connection)),
        };
        Ok(result)
    }
}

pub enum PlayResult<'a> {
    Again(Play<'a>),
    RoundOver(RoundOverview<'a>),
    GameOver(GameOverview<'a>),
}

pub enum DiscardResult<'a> {
    Again(Play<'a>),
    GameOver(GameOverview<'a>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HandCard {
    card: Option<Card>,
    selected: bool,
}

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};
    use crate::net::protocol::{Packet, Request, Response};
    use super::{HandCard, CurrentBlind};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PlayInfo {
        pub current_blind: CurrentBlind,
        pub hand: Vec<HandCard>
    }

    impl Response for PlayInfo {
    }

    impl Packet for PlayInfo {
        fn kind() -> String {
            "play/hand".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PlayClick {
        pub indices: Vec<u32>
    }

    impl Request for PlayClick {
        type Expect = Result<PlayInfo, String>;
    }

    impl Packet for PlayClick {
        fn kind() -> String {
            "play/click".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PlayPlay;

    impl Request for PlayPlay {
        type Expect = Result<PlayResult, String>;
    }

    impl Packet for PlayPlay {
        fn kind() -> String {
            "play/play".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PlayDiscard;

    impl Request for PlayDiscard {
        type Expect = Result<DiscardResult, String>;
    }

    impl Packet for PlayDiscard {
        fn kind() -> String {
            "play/discard".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub enum PlayResult {
        Again(PlayInfo),
        RoundOver(Vec<()>),
        GameOver(Vec<()>),
    }

    impl Response for PlayResult {
    }

    impl Packet for PlayResult {
        fn kind() -> String {
            "play/play/result".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub enum DiscardResult {
        Again(PlayInfo),
        GameOver(Vec<()>),
    }

    impl Response for DiscardResult {
    }

    impl Packet for DiscardResult {
        fn kind() -> String {
            "play/discard/result".to_string()
        }
    }
    
}