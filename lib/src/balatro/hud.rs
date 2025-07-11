use serde::{Deserialize, Serialize};
use super::{
    Error,
    consumables::Consumable,
    jokers::Joker,
};
use crate::balatro::{
    blinds::{BigBlindChoice, BossBlindChoice, SmallBlindChoice, Tag},
    play::PokerHand,
    menu::Stake,
    shop::VoucherKind,
    Screen,
    deck::PlayingCard,
};

#[allow(async_fn_in_trait)]
pub trait Hud<'a>: Sized + Screen<'a> {
    fn deck(&self) -> &[PlayingCard];
    fn hands(&self) -> u32;
    fn discards(&self) -> u32;
    fn round(&self) -> u32;
    fn ante(&self) -> u32;
    fn money(&self) -> u32;
    fn joker_slots(&self) -> u32;
    fn jokers(&self) -> &[Joker];
    fn tags(&self) -> &[Tag];
    fn run_info(&self) -> &RunInfo;
    async fn move_joker(self, from: u32, to: u32) -> Result<Self, Error>;
    async fn sell_joker(self, index: u32) -> Result<Self, Error>;
    fn consumable_slots(&self) -> u32;
    fn consumables(&self) -> &[Consumable];
    async fn move_consumable(self, from: u32, to: u32) -> Result<Self, Error>;
    async fn use_consumable(self, index: u32) -> Result<Self, Error>;
    async fn sell_consumable(self, index: u32) -> Result<Self, Error>;
}

#[macro_export]
macro_rules! impl_hud {
    ($($t:ident),*) => {
        $(
            impl<'a> $crate::balatro::hud::Hud<'a> for $t<'a> {
                fn hands(&self) -> u32 {
                    self.info.hud.hands
                }

                fn deck(&self) -> &[PlayingCard] {
                    &self.info.hud.deck
                }

                fn discards(&self) -> u32 {
                    self.info.hud.discards
                }

                fn round(&self) -> u32 {
                    self.info.hud.round
                }

                fn ante(&self) -> u32 {
                    self.info.hud.ante
                }

                fn money(&self) -> u32 {
                    self.info.hud.money
                }

                fn joker_slots(&self) -> u32 {
                    self.info.hud.joker_slots
                }

                fn jokers(&self) -> &[$crate::balatro::jokers::Joker] {
                    &self.info.hud.jokers
                }

                fn tags(&self) -> &[$crate::balatro::blinds::Tag] {
                    &self.info.hud.tags
                }

                fn run_info(&self) -> &$crate::balatro::hud::RunInfo {
                    &self.info.hud.run_info
                }

                async fn move_joker(self, from: u32, to: u32) -> Result<Self, $crate::balatro::Error> {
                    let new_info = self
                        .connection
                        .request($crate::balatro::hud::protocol::MoveJoker { from, to, _marker: std::marker::PhantomData::<&$t> })
                        .await??;
                    Ok(Self::new(new_info, self.connection))
                }

                async fn sell_joker(self, index: u32) -> Result<Self, $crate::balatro::Error> {
                    let new_info = self
                        .connection
                        .request($crate::balatro::hud::protocol::SellJoker { index, _marker: std::marker::PhantomData::<&$t> })
                        .await??;
                    Ok(Self::new(new_info, self.connection))
                }

                fn consumable_slots(&self) -> u32 {
                    self.info.hud.consumable_slots
                }

                fn consumables(&self) -> &[$crate::balatro::consumables::Consumable] {
                    &self.info.hud.consumables
                }

                async fn move_consumable(self, from: u32, to: u32) -> Result<Self, $crate::balatro::Error> {
                    let new_info = self
                        .connection
                        .request($crate::balatro::hud::protocol::MoveConsumable { from, to, _marker: std::marker::PhantomData::<&$t> })
                        .await??;
                    Ok(Self::new(new_info, self.connection))
                }

                async fn use_consumable(self, index: u32) -> Result<Self, $crate::balatro::Error> {
                    let new_info = self
                        .connection
                        .request($crate::balatro::hud::protocol::UseConsumable { index, _marker: std::marker::PhantomData::<&$t> })
                        .await??;
                    Ok(Self::new(new_info, self.connection))
                }

                async fn sell_consumable(self, index: u32) -> Result<Self, $crate::balatro::Error> {
                    let new_info = self
                        .connection
                        .request($crate::balatro::hud::protocol::SellConsumable { index, _marker: std::marker::PhantomData::<&$t> })
                        .await??;
                    Ok(Self::new(new_info, self.connection))
                }
            }
        )*
    };
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RunInfo {
    pub poker_hands: CurrentPokerHands,
    pub blinds: CurrentBlinds,
    pub vouchers_redeemed: Vec<VoucherKind>,
    pub stake: Stake
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CurrentPokerHands {
    pub high_card: CurrentPokerHand,
    pub pair: CurrentPokerHand,
    pub two_pair: CurrentPokerHand,
    pub three_of_a_kind: CurrentPokerHand,
    pub straight: CurrentPokerHand,
    pub flush: CurrentPokerHand,
    pub full_house: CurrentPokerHand,
    pub four_of_a_kind: CurrentPokerHand,
    pub straight_flush: CurrentPokerHand,
    pub five_of_a_kind: Option<CurrentPokerHand>,
    pub flush_house: Option<CurrentPokerHand>,
    pub flush_fives: Option<CurrentPokerHand>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CurrentPokerHand {
    pub hand: PokerHand,
    pub played: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CurrentBlinds {
    pub small: SmallBlindChoice,
    pub big: BigBlindChoice,
    pub boss: BossBlindChoice,
}

pub(crate) mod protocol {
    use crate::{
        balatro::{
        blinds::Tag,
        consumables::Consumable,
        hud::RunInfo,
        jokers::Joker,
        deck::PlayingCard
        },
        net::protocol::{Packet, Request, Response}
    };
    use serde::{Deserialize, Serialize};
    use super::Hud;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct HudInfo {
        pub hands: u32,
        pub discards: u32,
        pub round: u32,
        pub ante: u32,
        pub money: u32,
        pub joker_slots: u32,
        pub jokers: Vec<Joker>,
        pub tags: Vec<Tag>,
        pub consumable_slots: u32,
        pub consumables: Vec<Consumable>,
        pub run_info: RunInfo,
        pub deck: Vec<PlayingCard>,
    }

    impl Response for HudInfo {}

    impl Packet for HudInfo {
        fn kind() -> String {
            "hud/info".to_string()
        }
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct MoveJoker<'a, S: Hud<'a>> {
        pub from: u32,
        pub to: u32,
        pub _marker: std::marker::PhantomData<&'a S>,
    }

    impl<'a, S: Hud<'a>> Request for MoveJoker<'a, S> {
        type Expect = Result<S::Info, String>;
    }

    impl<'a, S: Hud<'a>> Packet for MoveJoker<'a, S> {
        fn kind() -> String {
            format!("{}/hud/jokers/move", S::name())
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct SellJoker<'a, S: Hud<'a>> {
        pub index: u32,
        pub _marker: std::marker::PhantomData<&'a S>,
    }

    impl<'a, S: Hud<'a>> Request for SellJoker<'a, S> {
        type Expect = Result<S::Info, String>;
    }

    impl<'a, S: Hud<'a>> Packet for SellJoker<'a, S> {
        fn kind() -> String {
            format!("{}/hud/jokers/sell", S::name())
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct MoveConsumable<'a, S: Hud<'a>> {
        pub from: u32,
        pub to: u32,
        pub _marker: std::marker::PhantomData<&'a S>,
    }

    impl<'a, S: Hud<'a>> Request for MoveConsumable<'a, S> {
        type Expect = Result<S::Info, String>;
    }

    impl<'a, S: Hud<'a>> Packet for MoveConsumable<'a, S> {
        fn kind() -> String {
            format!("{}/hud/consumables/move", S::name())
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct UseConsumable<'a, S: Hud<'a>> {
        pub index: u32,
        pub _marker: std::marker::PhantomData<&'a S>,
    }

    impl<'a, S: Hud<'a>> Request for UseConsumable<'a, S> {
        type Expect = Result<S::Info, String>;
    }

    impl<'a, S: Hud<'a>> Packet for UseConsumable<'a, S> {
        fn kind() -> String {
            format!("{}/hud/consumables/use", S::name())
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct SellConsumable<'a, S: Hud<'a>> {
        pub index: u32,
        pub _marker: std::marker::PhantomData<&'a S>,
    }

    impl<'a, S: Hud<'a>> Request for SellConsumable<'a, S> {
        type Expect = Result<S::Info, String>;
    }

    impl<'a, S: Hud<'a>> Packet for SellConsumable<'a, S> {
        fn kind() -> String {
            format!("{}/hud/consumables/sell", S::name())
        }
    }
}
