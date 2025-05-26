use super::Error;
use super::consumables::Consumable;
use super::jokers::Joker;
use super::overview::GameOverview;
use crate::balatro::Screen;

#[allow(async_fn_in_trait)]
pub trait Hud<'a>: Sized + Screen<'a> {
    fn hands(&self) -> u32;

    fn discards(&self) -> u32;

    fn round(&self) -> u32;

    fn ante(&self) -> u32;

    fn money(&self) -> u32;

    fn jokers(&self) -> &[Joker];

    async fn move_joker(self, from: u32, to: u32) -> Result<Self, Error>;

    async fn sell_joker(self, index: u32) -> Result<Self, Error>;

    fn consumables(&self) -> &[Consumable];

    async fn move_consumable(self, from: u32, to: u32) -> Result<Self, Error>;

    async fn use_consumable(self, index: u32) -> Result<UseConsumableResult<'a, Self>, Error>;

    async fn sell_consumable(self, index: u32) -> Result<Self, Error>;
}

pub enum UseConsumableResult<'a, S: Hud<'a>> {
    Used(S),
    GameOver(GameOverview<'a>),
}

#[macro_export]
macro_rules! impl_hud {
    ($($t:ident),*) => {
        $(
            impl<'a> $crate::balatro::hud::Hud<'a> for $t<'a> {
                fn hands(&self) -> u32 {
                    self.info.hud.hands
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

                fn jokers(&self) -> &[$crate::balatro::jokers::Joker] {
                    &self.info.hud.jokers
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

                async fn use_consumable(self, index: u32) -> Result<$crate::balatro::hud::UseConsumableResult<'a, Self>, $crate::balatro::Error> {
                    let new_info = self
                        .connection
                        .request($crate::balatro::hud::protocol::UseConsumable { index, _marker: std::marker::PhantomData::<&$t> })
                        .await??;
                    match new_info {
                        $crate::balatro::hud::protocol::UseConsumableResult::Used(new_info) => Ok($crate::balatro::hud::UseConsumableResult::Used(Self::new(new_info, self.connection))),
                        $crate::balatro::hud::protocol::UseConsumableResult::GameOver => Ok($crate::balatro::hud::UseConsumableResult::GameOver($crate::balatro::overview::GameOverview::new(self.connection))),
                    }
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

pub(crate) mod protocol {
    use crate::balatro::consumables::Consumable;
    use crate::balatro::jokers::Joker;
    use crate::net::protocol::{Packet, Request, Response};
    use serde::{Deserialize, Serialize};

    use super::Hud;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct HudInfo {
        pub hands: u32,
        pub discards: u32,
        pub round: u32,
        pub ante: u32,
        pub money: u32,
        pub jokers: Vec<Joker>,
        pub consumables: Vec<Consumable>,
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
        type Expect = Result<UseConsumableResult<'a, S>, String>;
    }

    impl<'a, S: Hud<'a>> Packet for UseConsumable<'a, S> {
        fn kind() -> String {
            format!("{}/hud/consumables/use", S::name())
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum UseConsumableResult<'a, S: Hud<'a>> {
        Used(S::Info),
        GameOver,
    }

    impl<'a, S: Hud<'a>> Response for UseConsumableResult<'a, S> {}

    impl<'a, S: Hud<'a>> Packet for UseConsumableResult<'a, S> {
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
