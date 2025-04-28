use crate::net::Connection;
use protocol::BlindInfo;
use serde::{Deserialize, Serialize};
use super::hand::SelectHand;

pub struct SelectBlind<'a> {
    info: protocol::BlindInfo,
    connection: &'a mut Connection,
}

impl<'a> SelectBlind<'a> {
    pub(crate) fn new(info: BlindInfo, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }

    pub async fn select(self) -> Result<SelectHand<'a>, super::Error> {
        let info = self.connection.request(protocol::SelectBlind).await??;
        Ok(SelectHand::new(info, self.connection))
    }

    pub async fn skip(self) -> Result<SelectBlind<'a>, super::Error> {
        let info = self.connection.request(protocol::SkipBlind).await??;
        Ok(Self { info, connection: self.connection })
    }

    pub fn small(&self) -> &SmallBlind {
        &self.info.small
    }

    pub fn big(&self) -> &BigBlind {
        &self.info.big
    }

    pub fn boss(&self) -> &BossBlind {
        &self.info.boss
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SmallBlind {
    pub state: BlindState,
    pub chips: u32,
    pub tag: Tag,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BigBlind {
    pub state: BlindState,
    pub chips: u32,
    pub tag: Tag,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Tag {
    #[serde(rename = "tag_uncommon")]
    Uncommon,
    #[serde(rename = "tag_rare")]
    Rare,
    #[serde(rename = "tag_negative")]
    Negative,
    #[serde(rename = "tag_foil")]
    Foil,
    #[serde(rename = "tag_holographic")]
    Holographic,
    #[serde(rename = "tag_polychrome")]
    Polychrome,
    #[serde(rename = "tag_investment")]
    Investment,
    #[serde(rename = "tag_voucher")]
    Voucher,
    #[serde(rename = "tag_boss")]
    Boss,
    #[serde(rename = "tag_standard")]
    Standard,
    #[serde(rename = "tag_charm")]
    Charm,
    #[serde(rename = "tag_meteor")]
    Meteor,
    #[serde(rename = "tag_buffoon")]
    Buffoon,
    #[serde(rename = "tag_handy")]
    Handy,
    #[serde(rename = "tag_ethereal")]
    Ethereal,
    #[serde(rename = "tag_coupon")]
    Coupon,
    #[serde(rename = "tag_double")]
    Double,
    #[serde(rename = "tag_juggle")]
    Juggle,
    #[serde(rename = "tag_d_six")]
    D6,
    #[serde(rename = "tag_topup")]
    TopUp,
    #[serde(rename = "tag_skip")]
    Skip,
    #[serde(rename = "tag_orbital")]
    Orbital,
    #[serde(rename = "tag_economy")]
    Economy,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BossBlind {
    pub kind: Boss,
    pub state: BlindState,
    pub chips: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Boss {
    #[serde(rename = "bl_ox")]
    TheOx,
    #[serde(rename = "bl_hook")]
    TheHook,
    #[serde(rename = "bl_mouth")]
    TheMouth,
    #[serde(rename = "bl_fish")]
    TheFish,
    #[serde(rename = "bl_club")]
    TheClub,
    #[serde(rename = "bl_manacle")]
    TheManacle,
    #[serde(rename = "bl_tooth")]
    TheTooth,
    #[serde(rename = "bl_wall")]
    TheWall,
    #[serde(rename = "bl_house")]
    TheHouse,
    #[serde(rename = "bl_mark")]
    TheMark,
    #[serde(rename = "bl_wheel")]
    TheWheel,
    #[serde(rename = "bl_arm")]
    TheArm,
    #[serde(rename = "bl_psychic")]
    ThePsychic,
    #[serde(rename = "bl_goad")]
    TheGoad,
    #[serde(rename = "bl_water")]
    TheWater,
    #[serde(rename = "bl_eye")]
    TheEye,
    #[serde(rename = "bl_plant")]
    ThePlant,
    #[serde(rename = "bl_needle")]
    TheNeedle,
    #[serde(rename = "bl_head")]
    TheHead,
    #[serde(rename = "bl_window")]
    TheWindow,
    #[serde(rename = "bl_serpent")]
    TheSerpent,
    #[serde(rename = "bl_pillar")]
    ThePillar,
    #[serde(rename = "bl_flint")]
    TheFlint,
    #[serde(rename = "bl_final_bell")]
    CeruleanBell,
    #[serde(rename = "bl_final_leaf")]
    VerdantLeaf,
    #[serde(rename = "bl_final_vessel")]
    VioletVessel,
    #[serde(rename = "bl_final_acorn")]
    AmberAcorn,
    #[serde(rename = "bl_final_heart")]
    CrimsonHeart,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum BlindState {
    Select,
    Skipped,
    Upcoming,
    Defeated,
}

pub(crate) mod protocol {
    use crate::{balatro::hand::protocol::HandInfo, net::protocol::{Packet, Request, Response}};
    use serde::{Deserialize, Serialize};

    use super::{BigBlind, BossBlind, SmallBlind};

    #[derive(Serialize, Deserialize)]
    pub struct BlindInfo {
        pub small: SmallBlind,
        pub big: BigBlind,
        pub boss: BossBlind,
    }

    impl Response for BlindInfo {}

    impl Packet for BlindInfo {
        fn kind() -> String {
            "blind_select/info".to_string()
        }
    }

    #[derive(Serialize)]
    pub struct SelectBlind;

    impl Request for SelectBlind {
        type Expect = Result<HandInfo, String>;
    }

    impl Packet for SelectBlind {
        fn kind() -> String {
            "blind_select/select".to_string()
        }
    }

    #[derive(Serialize)]
    pub struct SkipBlind;

    impl Request for SkipBlind {
        type Expect = Result<BlindInfo, String>;
    }

    impl Packet for SkipBlind {
        fn kind() -> String {
            "blind_select/skip".to_string()
        }
    }
}
