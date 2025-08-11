use serde::{Deserialize, Serialize};

use super::{
    Error, Screen,
    blinds::CurrentBlind,
    deck::PlayingCard,
    overview::{GameOverview, RoundOverview},
};
use crate::balatro_enum;
use crate::net::Connection;

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

    pub fn discarded(&self) -> &[PlayingCard] {
        &self.info.discarded
    }

    pub fn hand_size(&self) -> u32 {
        self.info.hand_size
    }

    pub fn score(&self) -> f64 {
        self.info.score
    }

    pub fn poker_hand(&self) -> Option<&PokerHand> {
        self.info.poker_hand.as_ref()
    }

    pub async fn click(self, indices: &[u32]) -> Result<Self, Error> {
        let info = self
            .connection
            .request(protocol::PlayClick {
                indices: indices.to_vec(),
            })
            .await??;
        Ok(Self::new(info, self.connection))
    }

    pub async fn play(self) -> Result<PlayResult<'a>, Error> {
        let info = self.connection.request(protocol::PlayPlay).await??;
        let result = match info {
            protocol::PlayResult::Again(info) => {
                PlayResult::Again(Self::new(info, self.connection))
            }
            protocol::PlayResult::RoundOver(info) => {
                PlayResult::RoundOver(RoundOverview::new(info, self.connection))
            }
            protocol::PlayResult::GameOver(info) => {
                PlayResult::GameOver(GameOverview::new(info, self.connection))
            }
        };
        Ok(result)
    }

    pub async fn discard(self) -> Result<DiscardResult<'a>, Error> {
        let info = self.connection.request(protocol::PlayDiscard).await??;
        let result = match info {
            protocol::DiscardResult::Again(info) => {
                DiscardResult::Again(Self::new(info, self.connection))
            }
            protocol::DiscardResult::GameOver(info) => {
                DiscardResult::GameOver(Box::new(GameOverview::new(*info, self.connection)))
            }
        };
        Ok(result)
    }

    pub async fn move_card(self, from: u32, to: u32) -> Result<Self, Error> {
        let info = self
            .connection
            .request(protocol::PlayMove { from, to })
            .await??;
        Ok(Self::new(info, self.connection))
    }
}

impl<'a> Screen<'a> for Play<'a> {
    type Info = protocol::PlayInfo;
    fn name() -> String {
        "play".to_string()
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

#[allow(clippy::large_enum_variant)]
pub enum DiscardResult<'a> {
    Again(Play<'a>),
    GameOver(Box<GameOverview<'a>>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HandCard {
    pub card: Option<PlayingCard>,
    pub selected: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PokerHand {
    pub kind: PokerHandKind,
    pub level: u64,
    pub chips: u64,
    pub mult: u64,
}

balatro_enum!(PokerHandKind {
    HighCard = "High Card",
    Pair = "Pair",
    TwoPair = "Two Pair",
    ThreeOfAKind = "Three of a Kind",
    Straight = "Straight",
    Flush = "Flush",
    FullHouse = "Full House",
    FourOfAKind = "Four of a Kind",
    StraightFlush = "Straight Flush",
    FiveOfAKind = "Five of a Kind",
    FlushHouse = "Flush House",
    FlushFive = "Flush Five"
});

pub(crate) mod protocol {
    use super::{CurrentBlind, HandCard, PokerHand};
    use crate::{
        balatro::{
            deck::PlayingCard,
            hud::protocol::HudInfo,
            overview::protocol::{GameOverviewInfo, RoundOverviewInfo},
        },
        net::protocol::{Packet, Request, Response},
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PlayInfo {
        pub current_blind: CurrentBlind,
        pub hand: Vec<HandCard>,
        pub score: f64,
        pub hand_size: u32,
        pub hud: HudInfo,
        pub poker_hand: Option<PokerHand>,
        pub discarded: Vec<PlayingCard>,
    }

    impl Response for PlayInfo {}

    impl Packet for PlayInfo {
        fn kind() -> String {
            "play/hand".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PlayClick {
        pub indices: Vec<u32>,
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

    #[allow(clippy::large_enum_variant)]
    #[derive(Serialize, Deserialize, Clone)]
    pub enum DiscardResult {
        Again(PlayInfo),
        GameOver(Box<GameOverviewInfo>),
    }

    impl Response for DiscardResult {}

    impl Packet for DiscardResult {
        fn kind() -> String {
            "play/discard/result".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PlayMove {
        pub from: u32,
        pub to: u32,
    }

    impl Request for PlayMove {
        type Expect = Result<PlayInfo, String>;
    }

    impl Packet for PlayMove {
        fn kind() -> String {
            "play/move".to_string()
        }
    }
}
