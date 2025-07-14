use serde::{Deserialize, Serialize};

use crate::balatro::deck::PlayingCard;
use crate::balatro::blinds::CurrentBlind;
use crate::balatro::menu::{self, Menu, Seed};
use crate::balatro::play::PokerHandKind;
use crate::net::Connection;
use crate::balatro::{
    Error,
    shop::Shop,
};

use super::blinds::Tag;
use super::jokers::JokerKind;
use super::Screen;
pub struct RoundOverview<'a> {
    connection: &'a mut Connection,
    info: protocol::RoundOverviewInfo,
}

impl<'a> RoundOverview<'a> {
    pub fn earnings(&self) -> Vec<Earning> {
        self.info.earnings.iter().map(|e| {
            let kind = match e.kind.clone() {
                protocol::EarningKind::Joker(k) => EarningKind::Joker(k),
                protocol::EarningKind::Tag(t) => EarningKind::Tag(t),
                protocol::EarningKind::Blind(_) => EarningKind::Blind,
                protocol::EarningKind::Interest(_) => EarningKind::Interest,
                protocol::EarningKind::Hands(h) => EarningKind::Hands(h),
                protocol::EarningKind::Discards(d) => EarningKind::Discards(d),
            };
            Earning { kind, value: e.value }
        }).collect()
    }

    pub fn total_earned(&self) -> u64 {
        self.info.total_earned
    }

    pub async fn cash_out(self) -> Result<Shop<'a>, Error> {
        let info = self.connection.request(protocol::CashOut).await??;
        Ok(Shop::new(info, self.connection))
    }
}

impl<'a> Screen<'a> for RoundOverview<'a> {
    type Info = protocol::RoundOverviewInfo;
    fn name() -> String {
        "overview".to_string()
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

crate::impl_hud!(RoundOverview);

#[derive(Clone, Debug)]
pub struct Earning {
    pub kind: EarningKind,
    pub value: u64,
}

#[derive(Clone, Debug)]
pub enum EarningKind {
    Joker(JokerKind),
    Tag(Tag),
    Blind,
    Interest,
    Hands(u64),
    Discards(u64),
}

pub struct GameOverview<'a> {
    connection: &'a mut Connection,
    info: protocol::GameOverviewInfo,
}

impl<'a> GameOverview<'a> {
    pub(crate) fn new(info: protocol::GameOverviewInfo, connection: &'a mut Connection) -> Self {
        Self { connection, info }
    }

    pub fn outcome(&self) -> &Outcome {
        &self.info.outcome
    }

    pub fn best_hand(&self) -> Option<u64> {
        self.info.best_hand
    }

    pub fn most_played_hand(&self) -> MostPlayedHand {
        self.info.most_played_hand
    }

    pub fn cards_played(&self) -> u64 {
        self.info.cards_played
    }

    pub fn cards_discarded(&self) -> u64 {
        self.info.cards_discarded
    }

    pub fn cards_purchased(&self) -> u64 {
        self.info.cards_purchased
    }

    pub fn times_rerolled(&self) -> u64 {
        self.info.times_rerolled
    }

    pub fn new_discoveries(&self) -> u64 {
        self.info.new_discoveries
    }

    pub fn seed(&self) -> &Seed {
        &self.info.seed
    }

    pub fn menu(self) -> Menu<'a> {
        Menu::new(self.connection, menu::protocol::MenuInfo { saved_run: None })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Outcome {
    Win,
    Loss { defeated_by: CurrentBlind, round: u64, ante: u64 },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MostPlayedHand {
    pub kind: PokerHandKind,
    pub times_played: u64,
}

pub(crate) mod protocol {
    use crate::{
        balatro::{hud::protocol::HudInfo, jokers::JokerKind, menu::Seed, overview::{MostPlayedHand, Outcome, Tag}, shop::protocol::ShopInfo}, net::protocol::{Packet, Request, Response}
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct RoundOverviewInfo {
        pub hud: HudInfo,
        pub earnings: Vec<Earning>,
        pub total_earned: u64,
    }

    impl Response for RoundOverviewInfo {}

    impl Packet for RoundOverviewInfo {
        fn kind() -> String {
            "overview/round".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Earning {
        pub kind: EarningKind,
        pub value: u64,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub enum EarningKind {
        Joker(JokerKind),
        Tag(Tag),
        Blind(Vec<()>),
        Interest(Vec<()>),
        Hands(u64),
        Discards(u64),
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct CashOut;

    impl Request for CashOut {
        type Expect = Result<ShopInfo, String>;
    }

    impl Packet for CashOut {
        fn kind() -> String {
            "overview/cash_out".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GameOverviewInfo {
        pub outcome: Outcome,
        pub best_hand: Option<u64>,
        pub most_played_hand: MostPlayedHand,
        pub cards_played: u64,
        pub cards_discarded: u64,
        pub cards_purchased: u64,
        pub times_rerolled: u64,
        pub new_discoveries: u64,
        pub seed: Seed,
    }

    impl Response for GameOverviewInfo {}

    impl Packet for GameOverviewInfo {
        fn kind() -> String {
            "overview/game".to_string()
        }
    }
}
