use serde::{Deserialize, Serialize};
use crate::{balatro_enum, net::Connection,
    balatro::{
        Error,
        Joker,
        Tarot,
        Planet,
        Spectral,
        deck::Card,
        blinds::SelectBlind,
    }
};

pub struct Shop<'a> {
    info: protocol::ShopInfo,
    connection: &'a mut Connection,
}
impl<'a> Shop<'a> {
    pub(crate) fn new(info: protocol::ShopInfo, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
    pub fn main_cards(&self) -> &[MainCard] {
        &self.info.main
    }
    pub fn vouchers(&self) -> &[VoucherItem] {
        &self.info.vouchers
    }
    pub fn boosters(&self) -> &[BoosterItem] {
        &self.info.boosters
    }

    pub async fn buy_main(self, index: u8) -> Result<Self, Error> {
        let info = self.connection.request(protocol::ShopBuyMain { index: index }).await??;
        Ok(Self::new(info, self.connection))
    }
    pub async fn buy_and_use(self, index: u8) -> Result<Self, Error> {
        let info = self.connection.request(protocol::ShopBuyUse { index: index }).await??;
        Ok(Self::new(info, self.connection))
    }
    pub async fn buy_voucher(self, index: u8) -> Result<Self, Error> {
        let info = self.connection.request(protocol::ShopBuyVoucher { index: index }).await??;
        Ok(Self::new(info, self.connection))
    }
    pub async fn buy_booster(self, index: u8) -> Result<Self, Error> {
        let info = self.connection.request(protocol::ShopBuyBooster { index: index }).await??;
        Ok(Self::new(info, self.connection))
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MainCard {
    Joker(Joker),
    Planet(Planet),
    Tarot(Tarot),
    Spectral(Spectral),
    PlayingCard(Card),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoosterItem { booster:Booster }
balatro_enum!(Booster {
    ArcanaNormal = "p_arcana_normal",
    ArcanaMega = "p_arcana_mega",
    ArcanaJumbo = "p_arcana_jumbo",
    BuffoonNormal = "p_buffoon_normal",
    BuffoonMega = "p_buffoon_mega",
    BuffoonJumbo = "p_buffoon_jumbo",
    CelestialNormal = "p_celestial_normal",
    CelestialMega = "p_celestial_mega",
    CelestialJumbo = "p_celestial_jumbo",
    SpectralNormal = "p_spectral_normal",
    SpectralMega = "p_spectral_mega",
    SpectralJumbo = "p_spectral_jumbo",
    StandardNormal = "p_standard_normal",
    StandardMega = "p_standard_mega",
    StandardJumbo = "p_standard_jumbo",
});

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoucherItem { voucher:Voucher }
balatro_enum!(Voucher {
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
        net::protocol::{Packet, Request, Response},
        balatro::blinds::protocol::BlindInfo,
    };
    use super::{MainCard, VoucherItem, BoosterItem};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ShopInfo {
        pub main: Vec<MainCard>,
        pub vouchers: Vec<VoucherItem>,
        pub boosters: Vec<BoosterItem>,
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
    pub struct ShopBuyBooster {
        pub index: u8
    }

    impl Request for ShopBuyBooster {
        type Expect = Result<ShopInfo, String>;
    }

    impl Packet for ShopBuyBooster {
        fn kind() -> String {
            "shop/buybooster".to_string()
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
