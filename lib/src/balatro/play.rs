use serde::{Deserialize, Serialize};

use crate::net::Connection;
use super::{
    blinds::CurrentBlind, deck::PlayingCard, overview::{GameOverview, RoundOverview}, Screen, Error
};

pub struct Play<'a> {
    info: protocol::PlayInfo,
    connection: &'a mut Connection,
}

impl<'a> Play<'a> {
    pub fn blind(&self) -> &CurrentBlind {
        &self.info.current_blind
    }

    pub fn hand(&self) -> &[HandCard] {
        &self.info.hand
    }

    pub fn score(&self) -> f64 {
        self.info.score
    }
    
    pub fn poker_hand(&self) -> Option<&PokerHand> {
        self.info.poker_hand.as_ref()
    }

    pub async fn click(self, indices: &[u32]) -> Result<Self, Error> {
        let info = self.connection.request(protocol::PlayClick { indices: indices.to_vec() }).await??;
        Ok(Self::new(info, self.connection))
    }

    pub async fn play(self) -> Result<PlayResult<'a>, Error> {
        let info = self.connection.request(protocol::PlayPlay).await??;
        let result = match info {
            protocol::PlayResult::Again(info) => PlayResult::Again(Self::new(info, self.connection)),
            protocol::PlayResult::RoundOver(info) => PlayResult::RoundOver(RoundOverview::new(info, self.connection)),
            protocol::PlayResult::GameOver(info) => PlayResult::GameOver(GameOverview::new(self.connection, info)),
        };
        Ok(result)
    }

    pub async fn discard(self) -> Result<DiscardResult<'a>, Error> {
        let info = self.connection.request(protocol::PlayDiscard).await??;
        let result = match info {
            protocol::DiscardResult::Again(info) => DiscardResult::Again(Self::new(info, self.connection)),
            protocol::DiscardResult::GameOver(info) => DiscardResult::GameOver(GameOverview::new(self.connection, info)),
        };
        Ok(result)
    }
}

impl<'a> Screen<'a> for Play<'a> {
    type Info = protocol::PlayInfo;
    fn name() -> &'static str {
        "play"
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

crate::impl_hud!(Play);

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
    card: Option<PlayingCard>,
    selected: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PokerHand {
    kind: PokerHandKind,
    level: u64,
    chips: u64,
    mult: u64,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PokerHandKind {
    #[serde(rename = "High Card")]
    HighCard,
    #[serde(rename = "Pair")]
    Pair,
    #[serde(rename = "Two Pair")]
    TwoPair,
    #[serde(rename = "Three of a Kind")]
    ThreeOfAKind,
    #[serde(rename = "Straight")]
    Straight,
    #[serde(rename = "Flush")]
    Flush,
    #[serde(rename = "Full House")]
    FullHouse,
    #[serde(rename = "Four of a Kind")]
    FourOfAKind,
    #[serde(rename = "Straight Flush")]
    StraightFlush,
    #[serde(rename = "Five of a Kind")]
    FiveOfAKind,
    #[serde(rename = "Flush House")]
    FlushHouse,
    #[serde(rename = "Flush Five")]
    FlushFive,
}

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};
    use crate::net::protocol::{Packet, Request, Response};
    use super::{CurrentBlind, HandCard, PokerHand};
    use crate::balatro::overview::protocol::{GameOverviewInfo, RoundOverviewInfo};
    use crate::balatro::hud::protocol::HudInfo;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PlayInfo {
        pub current_blind: CurrentBlind,
        pub hand: Vec<HandCard>,
        pub score: f64,
        pub hud: HudInfo,
        pub poker_hand: Option<PokerHand>,
    }

    impl Response for PlayInfo {}

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
        RoundOver(RoundOverviewInfo),
        GameOver(GameOverviewInfo),
    }

    impl Response for PlayResult {}

    impl Packet for PlayResult {
        fn kind() -> String {
            "play/play/result".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub enum DiscardResult {
        Again(PlayInfo),
        GameOver(GameOverviewInfo),
    }

    impl Response for DiscardResult {}

    impl Packet for DiscardResult {
        fn kind() -> String {
            "play/discard/result".to_string()
        }
    }
    
}
