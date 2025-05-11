use serde::{Deserialize, Serialize};

use crate::net::Connection;

pub struct Shop<'a> {
    info: protocol::ShopInfo,
    connection: &'a mut Connection,
}
impl<'a> Shop<'a> {
    pub(crate) fn new(info: protocol::ShopInfo, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
    pub async fn buy_main(self, index: u8) -> Result<Self, Error> {
        let info = self.connection.request(protocol::BuyMain { index: index }).await;
        Ok(Self::new(info, self.connection))
    }
    pub async fn buy_and_use(self, index: u8) -> Result<Self, Error> {
        let info = self.connection.request(protocol::BuyUse { index: index }).await;
        Ok(Self::new(info, self.connection))
    }
    pub async fn buy_voucher(self, index: u8) -> Result<Self, Error> {
        let info = self.connection.request(protocol::BuyVoucher { index: index }).await;
        Ok(Self::new(info, self.connection))
    }
    pub async fn buy_booster(self, index: u8) -> Result<Self, Error> {
        let info = self.connection.request(protocol::BuyBooster { index: index }).await;
        Ok(Self::new(info, self.connection))
    }
}

#[macro_export]
macro_rules! balatro_enum {
    ($name:ident { $($item:ident = $identifier:literal),* $(,)? }) => {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        enum $name {
            $(
                #[serde(rename = $identifier)]
                $item,
            )*
        }
    };
}

balatro_enum!(Boosters {
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
balatro_enum!(Vouchers {
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

#[derive(Serialize, Deserialize, Clone, Debug)]

pub(crate) mod protocol {
    use serde::{Deserialize, Serialize};
    use crate::net::protocol::{Packet, Request, Response};
    use super::{HandCard, CurrentBlind};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ShopInfo {
        pub jokers: ,
        pub vouchers: ,
        pub boosters: ,
    }

    impl Response for ShopInfo {
    }

    impl Packet for ShopInfo {
        fn kind() -> String {
            "shop/info".to_string()
        }
    }
}
