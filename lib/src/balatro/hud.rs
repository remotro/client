use protocol::HudCompatible;

use super::consumables::Consumable;
use super::jokers::Joker;
use super::Error;
use crate::balatro::protocol::NewScreen;
use crate::net::Connection;

pub struct Hud<'a, I: HudCompatible> {
    connection: &'a mut Connection,
    info: I,
}

impl<'a, I: HudCompatible> Hud<'a, I> {
    pub(crate) fn new(info: I, connection: &'a mut Connection) -> Self {
        Self { connection, info }
    }

    pub fn back(self) -> I::Screen {
        I::Screen::new(self.info, self.connection)
    }

    pub fn hands(&self) -> u32 {
        self.info.hud_info().hands
    }

    pub fn discards(&self) -> u32 {
        self.info.hud_info().discards
    }

    pub fn round(&self) -> u32 {
        self.info.hud_info().round
    }

    pub fn ante(&self) -> u32 {
        self.info.hud_info().ante
    }

    pub fn money(&self) -> u32 {
        self.info.hud_info().money
    }

    pub fn jokers(&self) -> &[Joker] {
        &self.info.hud_info().jokers
    }

    pub async fn move_joker(self, from: u32, to: u32) -> Result<Self, Error> {
        let new_info = self
            .connection
            .request(protocol::MoveJoker { from, to, _marker: std::marker::PhantomData })
            .await??;
        Ok(Self::new(new_info, self.connection))
    }

    pub async fn sell_joker(self, index: u32) -> Result<Self, Error> {
        let new_info = self
            .connection
            .request(protocol::SellJoker { index, _marker: std::marker::PhantomData })
            .await??;
        Ok(Self::new(new_info, self.connection))
    }

    pub fn consumables(&self) -> &[Consumable] {
        &self.info.hud_info().consumables
    }

    pub async fn move_consumable(self, from: u32, to: u32) -> Result<Self, Error> {
        let new_info = self
            .connection
            .request(protocol::MoveConsumable { from, to, _marker: std::marker::PhantomData })
            .await??;
        Ok(Self::new(new_info, self.connection))
    }

    pub async fn use_consumable(self, index: u32) -> Result<Self, Error> {
        let new_info = self
            .connection
            .request(protocol::UseConsumable { index, _marker: std::marker::PhantomData })
            .await??;
        Ok(Self::new(new_info, self.connection))
    }

    pub async fn sell_consumable(self, index: u32) -> Result<Self, Error> {
        let new_info = self
            .connection
            .request(protocol::SellConsumable { index, _marker: std::marker::PhantomData })
            .await??;
        Ok(Self::new(new_info, self.connection))
    }
}

pub mod protocol {
    use crate::balatro::jokers::Joker;
    use crate::balatro::consumables::Consumable;
    use crate::balatro::protocol::NewScreen;
    use crate::net::protocol::{Packet, Request, Response};
    use serde::{Deserialize, Serialize};

    pub trait HudCompatible: Response {
        type Screen: NewScreen<Info = Self>;
        fn kind_prefix() -> &'static str;
        fn hud_info(&self) -> &HudInfo;
    }

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
    pub struct MoveJoker<I: HudCompatible> {
        pub from: u32,
        pub to: u32,
        pub _marker: std::marker::PhantomData<I>,
    }

    impl<I: HudCompatible> Request for MoveJoker<I> {
        type Expect = Result<I, String>;
    }

    impl<I: HudCompatible> Packet for MoveJoker<I> {
        fn kind() -> String {
            format!("hud/jokers/move/{}", I::kind_prefix())
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct SellJoker<I: HudCompatible> {
        pub index: u32,
        pub _marker: std::marker::PhantomData<I>,
    }

    impl<I: HudCompatible> Request for SellJoker<I> {
        type Expect = Result<I, String>;
    }

    impl<I: HudCompatible> Packet for SellJoker<I> {
        fn kind() -> String {
            format!("hud/jokers/sell/{}", I::kind_prefix())
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct MoveConsumable<I: HudCompatible> {
        pub from: u32,
        pub to: u32,
        pub _marker: std::marker::PhantomData<I>,
    }

    impl<I: HudCompatible> Request for MoveConsumable<I> {
        type Expect = Result<I, String>;
    }

    impl<I: HudCompatible> Packet for MoveConsumable<I> {
        fn kind() -> String {
            format!("hud/consumables/move/{}", I::kind_prefix())
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct UseConsumable<I: HudCompatible> {
        pub index: u32,
        pub _marker: std::marker::PhantomData<I>,
    }

    impl<I: HudCompatible> Request for UseConsumable<I> {
        type Expect = Result<I, String>;
    }

    impl<I: HudCompatible> Packet for UseConsumable<I> {
        fn kind() -> String {
            format!("hud/consumables/use/{}", I::kind_prefix())
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct SellConsumable<I: HudCompatible> {
        pub index: u32,
        pub _marker: std::marker::PhantomData<I>,
    }

    impl<I: HudCompatible> Request for SellConsumable<I> {
        type Expect = Result<I, String>;
    }

    impl<I: HudCompatible> Packet for SellConsumable<I> {
        fn kind() -> String {
            format!("hud/consumables/sell/{}", I::kind_prefix())
        }
    }
}