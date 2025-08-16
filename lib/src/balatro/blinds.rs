use crate::{balatro::{boosters, translations::{Translatable, Translation, Translations}, Collection}, balatro_enum, net::Connection, render};
use serde::{Deserialize, Serialize};
use super::{play::{Play, PokerHandKind}, Screen};

pub struct SelectBlind<'a> {
    info: protocol::BlindInfo,
    connection: &'a mut Connection,
}

impl<'a> SelectBlind<'a> {
    pub async fn select(self) -> Result<Play<'a>, super::Error> {
        let info = self.connection.request(protocol::SelectBlind).await??;
        Ok(Play::new(info, self.connection))
    }

    pub async fn skip(self) -> Result<SkipResult<'a>, super::Error> {
        let info = self.connection.request(protocol::SkipBlind::<'a> { _r_marker: std::marker::PhantomData }).await??;
        match info {
            protocol::SkipBlindResult::Select(info) => Ok(SkipResult::Select(SelectBlind::new(info, self.connection))),
            protocol::SkipBlindResult::Booster(info) => match info {
                protocol::SkippedBooster::Arcana(info) => Ok(SkipResult::Booster(boosters::OpenBoosterPack::Arcana(boosters::OpenArcanaPack::new(info, self.connection)))),
                protocol::SkippedBooster::Buffoon(info) => Ok(SkipResult::Booster(boosters::OpenBoosterPack::Buffoon(boosters::OpenBuffoonPack::new(info, self.connection)))),
                protocol::SkippedBooster::Celestial(info) => Ok(SkipResult::Booster(boosters::OpenBoosterPack::Celestial(boosters::OpenCelestialPack::new(info, self.connection)))),
                protocol::SkippedBooster::Spectral(info) => Ok(SkipResult::Booster(boosters::OpenBoosterPack::Spectral(boosters::OpenSpectralPack::new(info, self.connection)))),
                protocol::SkippedBooster::Standard(info) => Ok(SkipResult::Booster(boosters::OpenBoosterPack::Standard(boosters::OpenStandardPack::new(info, self.connection)))),
            }
        }
    }

    pub fn small(&self) -> &SmallBlindChoice {
        &self.info.blinds.small
    }

    pub fn big(&self) -> &BigBlindChoice {
        &self.info.blinds.big
    }

    pub fn boss(&self) -> &BossBlindChoice {
        &self.info.blinds.boss
    }
}

impl<'a> Screen<'a> for SelectBlind<'a> {    
    type Info = protocol::BlindInfo;
    fn new(info: Self::Info, connection: &'a mut Connection) -> Self {
        Self { info, connection }
    }
    fn name() -> String {
        "blind_select".to_string()
    }
    async fn collection(self) -> Result<Collection, crate::balatro::Error> {
        let collection = self.connection.request(super::protocol::GetCollection).await??;
        Ok(collection.collection)
    }
}

crate::impl_hud!(SelectBlind);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CurrentBlind {
    Small { chips: u32 },
    Big { chips: u32 },
    Boss { kind: Boss, chips: u32 },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SmallBlindChoice {
    pub state: BlindState,
    pub chips: f64,
    pub tag: Tag,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BigBlindChoice {
    pub state: BlindState,
    pub chips: f64,
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
    Handy { earnings: u64 } = "tag_handy",
    Ethereal = "tag_ethereal",
    Coupon = "tag_coupon",
    Double = "tag_double",
    Juggle = "tag_juggle",
    D6 = "tag_d_six",
    TopUp = "tag_top_up",
    Skip { earnings: u64 } = "tag_skip",
    Orbital { hand: PokerHandKind } = "tag_orbital",
    Economy = "tag_economy",
    Garbage { earnings: u64 } = "tag_garbage",
});

impl Translatable for Tag {
    fn translate(&self, translations: &Translations) -> Translation {
        let path = format!["descriptions.Tag.{}", self.id()];
        match self {
            Self::Investment => render!(translations, path, 25).unwrap(),
            Self::Handy { earnings } => render!(translations, path, 1, *earnings).unwrap(),
            Self::Orbital { hand } => render!(translations, path, hand.translate(translations).name, 3).unwrap(),
            Self::Juggle => render!(translations, path, 3).unwrap(),
            Self::TopUp => render!(translations, path, 2).unwrap(),
            Self::Skip { earnings } => render!(translations, path, 5, *earnings).unwrap(),
            Self::Garbage { earnings } => render!(translations, path, 1, *earnings).unwrap(),
            _ => render!(translations, path).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BossBlindChoice {
    pub kind: Boss,
    pub state: BlindState,
    pub chips: f64,
}
balatro_enum!(Boss {
    TheOx { hand: PokerHandKind } = "bl_ox",
    TheHook = "bl_hook",
    TheMouth = "bl_mouth",
    TheFish = "bl_fish",
    TheClub = "bl_club",
    TheManacle = "bl_manacle",
    TheTooth = "bl_tooth",
    TheWall = "bl_wall",
    TheHouse = "bl_house",
    TheMark = "bl_mark",
    TheWheel { probability: u64 }  = "bl_wheel",
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

impl Boss {
    pub fn is_finisher(&self) -> bool {
        matches!(self, Boss::CeruleanBell | Boss::VerdantLeaf | Boss::VioletVessel | Boss::AmberAcorn | Boss::CrimsonHeart)
    }
}

impl Translatable for Boss {
    fn translate(&self, translations: &Translations) -> Translation {
        let path = format!["descriptions.Blind.{}", self.id()];
        match self {
            Self::TheOx { hand } => render!(translations, path, hand.translate(translations).name).unwrap(),
            Self::TheWheel { probability } => {
                let translation = render!(translations, path).unwrap();
                Translation {
                    name: translation.name,
                    text: Some(probability.to_string() + &translation.text.unwrap())
                }
            }
            _ => render!(translations, path).unwrap()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum BlindState {
    Current,
    Select,
    Skipped,
    Upcoming,
    Defeated,
}

pub enum SkipResult<'a> {
    Select(SelectBlind<'a>),
    Booster(boosters::OpenBoosterPack<'a, protocol::SkipBlindResult<'a>>),
}

pub(crate) mod protocol {
    use crate::{balatro::{boosters, hud::protocol::HudInfo, play::protocol::PlayInfo, Screen}, net::protocol::{Packet, Request, Response}};
    use serde::{Deserialize, Serialize};

    use super::{BigBlindChoice, BossBlindChoice, SmallBlindChoice};

    #[derive(Serialize, Deserialize)]
    pub struct BlindInfo {
        pub hud: HudInfo,
        pub blinds: BlindChoices
    }

    impl Response for BlindInfo {}

    impl Packet for BlindInfo {
        fn kind() -> String {
            "blind_select/info".to_string()
        }
    }


    #[derive(Serialize, Deserialize)]
    pub struct BlindChoices {
        pub small: SmallBlindChoice,
        pub big: BigBlindChoice,
        pub boss: BossBlindChoice,
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

    #[derive(Deserialize)]
    pub enum SkipBlindResult<'a> {
        Select(BlindInfo),
        Booster(SkippedBooster<'a>),
    }

    impl<'a> Response for SkipBlindResult<'a> {}

    impl<'a> Packet for SkipBlindResult<'a> {
        fn kind() -> String {
            "blind_select/skip_result".to_string()
        }
    }

    #[derive(Deserialize)]
    pub enum SkippedBooster<'a> {
        Arcana(<boosters::OpenArcanaPack<'a, SkipBlindResult<'a>> as Screen<'a>>::Info),
        Buffoon(<boosters::OpenBuffoonPack<'a, SkipBlindResult<'a>> as Screen<'a>>::Info),
        Celestial(<boosters::OpenCelestialPack<'a, SkipBlindResult<'a>> as Screen<'a>>::Info),
        Spectral(<boosters::OpenSpectralPack<'a, SkipBlindResult<'a>> as Screen<'a>>::Info),
        Standard(<boosters::OpenStandardPack<'a, SkipBlindResult<'a>> as Screen<'a>>::Info),
    }


    #[derive(Serialize)]
    pub struct SkipBlind<'a> {
        pub _r_marker: std::marker::PhantomData<&'a ()>,
    }

    impl<'a> Request for SkipBlind<'a> {
        type Expect = Result<SkipBlindResult<'a>, String>;
    }

    impl<'a> Packet for SkipBlind<'a> {
        fn kind() -> String {
            "blind_select/skip".to_string()
        }
    }
}
