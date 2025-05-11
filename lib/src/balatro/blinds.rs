use crate::{balatro_enum, net::Connection};
use protocol::BlindInfo;
use serde::{Deserialize, Serialize};
use super::play::Play;

pub struct SelectBlind<'a> {
    info: protocol::BlindInfo,
    connection: &'a mut Connection,
}

impl<'a> SelectBlind<'a> {
    pub(crate) fn new(info: BlindInfo, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }

    pub async fn select(self) -> Result<Play<'a>, super::Error> {
        let info = self.connection.request(protocol::SelectBlind).await??;
        Ok(Play::new(info, self.connection))
    }

    pub async fn skip(self) -> Result<SelectBlind<'a>, super::Error> {
        let info = self.connection.request(protocol::SkipBlind).await??;
        Ok(Self { info, connection: self.connection })
    }

    pub fn small(&self) -> &SmallBlindChoice {
        &self.info.small
    }

    pub fn big(&self) -> &BigBlindChoice {
        &self.info.big
    }

    pub fn boss(&self) -> &BossBlindChoice {
        &self.info.boss
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CurrentBlind {
    Small { chips: u32 },
    Big { chips: u32 },
    Boss { kind: Boss, chips: u32 },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SmallBlindChoice {
    pub state: BlindState,
    pub chips: u32,
    pub tag: Tag,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BigBlindChoice {
    pub state: BlindState,
    pub chips: u32,
    pub tag: Tag,
}

balatro_enum!(Tag {
    Uncommon = "tag_uncommon",
    Rare = "tag_rare",
    Negative = "tag_negative",
    Foil = "tag_foil",
    Holographic = "tag_holo",
    Polychrome = "tag_polychrome",
    Investment = "tag_investment",
    Voucher = "tag_voucher",
    Boss = "tag_boss",
    Standard = "tag_standard",
    Charm = "tag_charm",
    Meteor = "tag_meteor",
    Buffoon = "tag_buffoon",
    Handy = "tag_handy",
    Ethereal = "tag_ethereal",
    Coupon = "tag_coupon",
    Double = "tag_double",
    Juggle = "tag_juggle",
    D6 = "tag_d_six",
    TopUp = "tag_topup",
    Skip = "tag_skip",
    Orbital = "tag_orbital",
    Economy = "tag_economy",
});

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BossBlindChoice {
    pub kind: Boss,
    pub state: BlindState,
    pub chips: u32,
}
balatro_enum!(Boss {
    TheOx = "bl_ox",
    TheHook = "bl_hook",
    TheMouth = "bl_mouth",
    TheFish = "bl_fish",
    TheClub = "bl_club",
    TheManacle = "bl_manacle",
    TheTooth = "bl_tooth",
    TheWall = "bl_wall",
    TheHouse = "bl_house",
    TheMark = "bl_mark",
    TheWheel = "bl_wheel",
    TheArm = "bl_arm",
    ThePsychic = "bl_psychic",
    TheGoad = "bl_goad",
    TheWater = "bl_water",
    TheEye = "bl_eye",
    ThePlant = "bl_plant",
    TheNeedle = "bl_needle",
    TheHead = "bl_head",
    TheWindow = "bl_window",
    TheSerpent = "bl_serpent",
    ThePillar = "bl_pillar",
    TheFlint = "bl_flint",
    CeruleanBell = "bl_final_bell",
    VerdantLeaf = "bl_final_leaf",
    VioletVessel = "bl_final_vessel",
    AmberAcorn = "bl_final_acorn",
    CrimsonHeart = "bl_final_heart",
});

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum BlindState {
    Select,
    Skipped,
    Upcoming,
    Defeated,
}

pub(crate) mod protocol {
    use crate::{balatro::play::protocol::PlayInfo, net::protocol::{Packet, Request, Response}};
    use serde::{Deserialize, Serialize};

    use super::{BigBlindChoice, BossBlindChoice, SmallBlindChoice};

    #[derive(Serialize, Deserialize)]
    pub struct BlindInfo {
        pub small: SmallBlindChoice,
        pub big: BigBlindChoice,
        pub boss: BossBlindChoice,
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
        type Expect = Result<PlayInfo, String>;
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
