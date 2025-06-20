use serde::{Deserialize, Serialize};
use crate::{balatro_enum, net::Connection,
    balatro::{
        Error,
        deck::{PlayingCard},
        blinds::SelectBlind,
    }
};
use super::{boosters::{BoosterPackKind, OpenBuffoonPack, OpenCelestialPack, OpenSpectralPack, OpenStandardPack, OpenArcanaPack}, consumables::{PlanetCard, SpectralCard, TarotCard}, jokers::Joker, Screen};

pub struct Shop<'a> {
    info: protocol::ShopInfo,
    connection: &'a mut Connection,
}

impl<'a> Shop<'a> {
    pub fn main_cards(&self) -> &[MainCard] {
        &self.info.main
    }

    pub fn vouchers(&self) -> &[Voucher] {
        &self.info.vouchers
    }

    pub fn boosters(&self) -> &[BoosterPack] {
        &self.info.boosters
    }

    pub async fn buy_main(self, index: u8) -> Result<Self, Error> {
        let info = self.connection.request(protocol::ShopBuyMain { index }).await??;
        Ok(Self::new(info, self.connection))
    }

    pub async fn buy_and_use(self, index: u8) -> Result<Self, Error> {
        let info = self.connection.request(protocol::ShopBuyUse { index }).await??;
        Ok(Self::new(info, self.connection))
    }

    pub async fn buy_voucher(self, index: u8) -> Result<Self, Error> {
        let info = self.connection.request(protocol::ShopBuyVoucher { index }).await??;
        Ok(Self::new(info, self.connection))
    }
    
    pub async fn buy_booster(self, index: u8) -> Result<BoughtBooster<'a>, Error> {
        let info = self.connection.request(protocol::ShopBuyBooster { index, _r_marker: std::marker::PhantomData }).await??;
        match info {
            protocol::BoughtBooster::Buffoon(info) => Ok(BoughtBooster::Buffoon(OpenBuffoonPack::new(info, self.connection))),
            protocol::BoughtBooster::Celestial(info) => Ok(BoughtBooster::Celestial(OpenCelestialPack::new(info, self.connection))),
            protocol::BoughtBooster::Spectral(info) => Ok(BoughtBooster::Spectral(OpenSpectralPack::new(info, self.connection))),
            protocol::BoughtBooster::Standard(info) => Ok(BoughtBooster::Standard(OpenStandardPack::new(info, self.connection))),
            protocol::BoughtBooster::Arcana(info) => Ok(BoughtBooster::Arcana(OpenArcanaPack::new(info, self.connection))),
        }
    }

    pub async fn reroll(self) -> Result<Self, Error> {
        let info = self.connection.request(protocol::ShopReroll {}).await??;
        Ok(Self::new(info, self.connection))
    }

    pub async fn leave(self) -> Result<SelectBlind<'a>, Error> {
        let info = self.connection.request(protocol::ShopContinue {}).await??;
        Ok(SelectBlind::new(info, self.connection))
    }
}

impl<'a> Screen<'a> for Shop<'a> {
    type Info = protocol::ShopInfo;
    fn name() -> &'static str {
        "shop"
    }
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
}

crate::impl_hud!(Shop);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MainCard {
    Joker(Joker),
    Planet(PlanetCard),
    Tarot(TarotCard),
    Spectral(SpectralCard),
    Playing(PlayingCard),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoosterPack { pub kind: BoosterPackKind, pub price: u8 }

pub enum BoughtBooster<'a> {
    Arcana(OpenArcanaPack<'a, Shop<'a>>),
    Buffoon(OpenBuffoonPack<'a, Shop<'a>>),
    Celestial(OpenCelestialPack<'a, Shop<'a>>),
    Spectral(OpenSpectralPack<'a, Shop<'a>>),
    Standard(OpenStandardPack<'a, Shop<'a>>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Voucher { kind: VoucherKind, price: u8 }

balatro_enum!(VoucherKind {
    Blank = "v_blank",
    Antimatter = "v_antimatter",
    ClearanceSale = "v_clearance_sale",
    Liquidation = "v_liquidation",
    CrystalBall = "v_crystal_ball",
    OmenGlobe = "v_omen_globe",
    DirectorsCut = "v_directors_cut",
    Retcon = "v_retcon",
    Hone = "v_hone",
    GlowUp = "v_glow_up",
    Grabber = "v_grabber",
    NachoTong = "v_nacho_tong",
    Hieroglyph = "v_hieroglyph",
    Petroglyph = "v_petroglyph",
    MagicTrick = "v_magic_trick",
    Illusion = "v_illusion",
    SeedMoney = "v_seed_money",
    MoneyTree = "v_money_tree",
    Telescope = "v_telescope",
    Observatory = "v_observatory",
    Overstock = "v_overstock_norm",
    OverstockPlus = "v_overstock_plus",
    PaintBrush = "v_paint_brush",
    Palette = "v_palette",
    PlanetMerchant = "v_planet_merchant",
    PlanetTycoon = "v_planet_tycoon",
    Wasteful = "v_wasteful",
    Recyclomancy = "v_recyclomancy",
    RerollSurplus = "v_reroll_surplus",
    RerollGlut = "v_reroll_glut",
    TarotMerchant = "v_tarot_merchant",
    TarotTycoon = "v_tarot_tycoon",
});

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};
    use crate::{
        balatro::{blinds::protocol::BlindInfo, boosters::{OpenBuffoonPack, OpenCelestialPack, OpenSpectralPack, OpenStandardPack, OpenArcanaPack}, hud::protocol::HudInfo, Screen}, net::protocol::{Packet, Request, Response}
    };
    use super::{BoosterPack, MainCard, Shop, Voucher};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ShopInfo {
        pub hud: HudInfo,
        pub main: Vec<MainCard>,
        pub vouchers: Vec<Voucher>,
        pub boosters: Vec<BoosterPack>,
    }

    impl Response for ShopInfo {}

    impl Packet for ShopInfo {
        fn kind() -> String {
            "shop/info".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ShopBuyMain {
        pub index: u8
    }

    impl Request for ShopBuyMain {
        type Expect = Result<ShopInfo, String>;
    }

    impl Packet for ShopBuyMain {
        fn kind() -> String {
            "shop/buymain".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ShopBuyUse {
        pub index: u8
    }

    impl Request for ShopBuyUse {
        type Expect = Result<ShopInfo, String>;
    }

    impl Packet for ShopBuyUse {
        fn kind() -> String {
            "shop/buyuse".to_string()
        }
    }
    
    #[derive(Serialize, Deserialize, Clone)]
    pub struct ShopBuyVoucher {
        pub index: u8
    }

    impl Request for ShopBuyVoucher {
        type Expect = Result<ShopInfo, String>;
    }

    impl Packet for ShopBuyVoucher {
        fn kind() -> String {
            "shop/buyvoucher".to_string()
        }
    }
    
    #[derive(Serialize, Deserialize, Clone)]
    pub struct ShopBuyBooster<'a> {
        pub index: u8,
        pub _r_marker: std::marker::PhantomData<&'a BoughtBooster<'a>>,
    }

    impl<'a> Request for ShopBuyBooster<'a> {
        type Expect = Result<BoughtBooster<'a>, String>;
    }

    impl Packet for ShopBuyBooster<'_> {
        fn kind() -> String {
            "shop/buybooster".to_string()
        }
    }

    #[derive(Deserialize)]
    pub enum BoughtBooster<'a> {
        Buffoon(<OpenBuffoonPack<'a, Shop<'a>> as Screen<'a>>::Info),
        Celestial(<OpenCelestialPack<'a, Shop<'a>> as Screen<'a>>::Info),
        Spectral(<OpenSpectralPack<'a, Shop<'a>> as Screen<'a>>::Info),
        Standard(<OpenStandardPack<'a, Shop<'a>> as Screen<'a>>::Info),
        Arcana(<OpenArcanaPack<'a, Shop<'a>> as Screen<'a>>::Info),
    }

    impl Response for BoughtBooster<'_> {}

    impl Packet for BoughtBooster<'_> {
        fn kind() -> String {
            "shop/bought_booster".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ShopReroll {}
    
    impl Request for ShopReroll {
        type Expect = Result<ShopInfo, String>;
    }

    impl Packet for ShopReroll {
        fn kind() -> String {
            "shop/reroll".to_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ShopContinue {}
    
    impl Request for ShopContinue {
        type Expect = Result<BlindInfo, String>;
    }

    impl Packet for ShopContinue {
        fn kind() -> String {
            "shop/continue".to_string()
        }
    }
}
