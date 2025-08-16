use serde::{Deserialize, Serialize};
use crate::{balatro::translations::{Translatable, Translation, Translations}, balatro_enum, render};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Consumable {
    Planet(PlanetCard),
    Tarot(TarotCard),
    Spectral(SpectralCard)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlanetCard {
    pub kind: PlanetKind,
    pub price: u64,
    pub negative: bool,
}

balatro_enum!(PlanetKind {
    Mercury { current_level: u64 } = "c_mercury",
    Venus { current_level: u64 } = "c_venus",
    Earth { current_level: u64 } = "c_earth",
    Mars { current_level: u64 } = "c_mars",
    Jupiter { current_level: u64 } = "c_jupiter",
    Saturn { current_level: u64 } = "c_saturn",
    Uranus { current_level: u64 } = "c_uranus",
    Neptune { current_level: u64 } = "c_neptune",
    Pluto { current_level: u64 } = "c_pluto",
    PlanetX { current_level: u64 } = "c_planet_x",
    Ceres { current_level: u64 } = "c_ceres",
    Eris { current_level: u64 } = "c_eris",
});

impl Translatable for PlanetKind {
    fn translate(&self, translations: &Translations) -> Translation {
        let path = format!["descriptions.Planet.{}", self.id()];
        match self {
            PlanetKind::Mercury { current_level} => render!(translations, path, *current_level, "Pair", 1, 15).unwrap(),
            PlanetKind::Venus { current_level } => render!(translations, path, *current_level, "Three of a Kind", 2, 20).unwrap(),
            PlanetKind::Earth { current_level } => render!(translations, path, *current_level, "Full House", 2, 25).unwrap(),
            PlanetKind::Mars { current_level } => render!(translations, path, *current_level, "Four of a Kind", 3, 30).unwrap(),
            PlanetKind::Jupiter { current_level } => render!(translations, path, *current_level, "Flush", 2, 15).unwrap(),
            PlanetKind::Saturn { current_level } => render!(translations, path, *current_level, "Saturn", 3, 30).unwrap(),
            PlanetKind::Uranus { current_level } => render!(translations, path, *current_level, "Two Pair", 1, 20).unwrap(),
            PlanetKind::Neptune { current_level } => render!(translations, path, *current_level, "Two Pair", 4, 40).unwrap(),
            PlanetKind::Pluto { current_level } => render!(translations, path, *current_level, "High Card", 1, 10).unwrap(),
            PlanetKind::PlanetX { current_level } => render!(translations, path, *current_level, "Five of a Kind", 3, 35).unwrap(),
            PlanetKind::Ceres { current_level } => render!(translations, path, *current_level, "Flush House", 4, 40).unwrap(),
            PlanetKind::Eris { current_level } => render!(translations, path, *current_level, "Flush Fives", 3, 50).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TarotCard {
    pub kind: TarotKind,
    pub price: u64,
    pub negative: bool,
}

balatro_enum!(TarotKind {
    Fool = "c_fool",
    Magician = "c_magician",
    HighPriestess = "c_high_priestess",
    Empress = "c_empress",
    Emperor = "c_emperor",
    Heirophant = "c_heirophant",
    Lovers = "c_lovers",
    Chariot = "c_chariot",
    Justice = "c_justice",
    Hermit = "c_hermit",
    WheelOfFortune { probability: u64 } = "c_wheel_of_fortune",
    Strength = "c_strength",
    HangedMan = "c_hanged_man",
    Death = "c_death",
    Temperance { earnings: u64 } = "c_temperance",
    Devil = "c_devil",
    Tower = "c_tower",
    Star = "c_star",
    Moon = "c_moon",
    Sun = "c_sun",
    Judgement = "c_judgement",
    World = "c_world",
});

impl Translatable for TarotKind {
    fn translate(&self, translations: &Translations) -> Translation {
        let path = format!["descriptions.Tarot.{}", self.id()];
        match self {
            TarotKind::Fool => render!(translations, path).unwrap(),
            TarotKind::Magician => render!(translations, path, 2, "Lucky Cards").unwrap(),
            TarotKind::HighPriestess => render!(translations, path, 2).unwrap(),
            TarotKind::Empress => render!(translations, path, 2, "Mult Cards").unwrap(),
            TarotKind::Emperor => render!(translations, path, 2).unwrap(),
            TarotKind::Heirophant => render!(translations, path, 2, "Bonus Cards").unwrap(),
            TarotKind::Lovers => render!(translations, path, 1, "Wild Card").unwrap(),
            TarotKind::Chariot => render!(translations, path, 1, "Steel Cards").unwrap(),
            TarotKind::Justice => render!(translations, path, 1, "Glass Card").unwrap(),
            TarotKind::Hermit => render!(translations, path, 20).unwrap(),
            TarotKind::WheelOfFortune { probability } => render!(translations, path, *probability).unwrap(),
            TarotKind::Strength => render!(translations, path, 2).unwrap(),
            TarotKind::HangedMan => render!(translations, path, 2).unwrap(),
            TarotKind::Death => render!(translations, path).unwrap(),
            TarotKind::Temperance { earnings } => render!(translations, path, *earnings).unwrap(),
            TarotKind::Devil => render!(translations, path, 3, "Gold Card").unwrap(),
            TarotKind::Tower => render!(translations, path, 3, "Stone Card").unwrap(),
            TarotKind::Star => render!(translations, path, 3, "Diamonds").unwrap(),
            TarotKind::Moon => render!(translations, path, 3, "Clubs").unwrap(),
            TarotKind::Sun => render!(translations, path, 3, "Hearts").unwrap(),
            TarotKind::Judgement => render!(translations, path).unwrap(),
            TarotKind::World => render!(translations, path, 3, "Spades").unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpectralCard {
    pub kind: SpectralKind,
    pub price: u64,
    pub negative: bool,
}

balatro_enum!(SpectralKind {
    Familiar = "c_familiar",
    Grim = "c_grim",
    Incantation = "c_incantation",
    Talisman = "c_talisman",
    Aura = "c_aura",
    Wraith = "c_wraith",
    Sigil = "c_sigil",
    Ouija = "c_ouija",
    Ectoplasm { hand_size_penalty: u64 } = "c_ectoplasm",
    Immolate = "c_immolate",
    Ankh = "c_ankh",
    DejaVu = "c_deja_vu",
    Hex = "c_hex",
    Trance = "c_trance",
    Medium = "c_medium",
    Cryptid = "c_cryptid",
    TheSoul = "c_soul",
    BlackHole = "c_black_hole",
});

impl Translatable for SpectralKind {
    fn translate(&self, translations: &Translations) -> Translation {
        let path = format!["descriptions.Spectral.{}", self.id()];
        match self {
            SpectralKind::Familiar => render!(translations, path, 3).unwrap(),
            SpectralKind::Grim => render!(translations, path, 3).unwrap(),
            SpectralKind::Incantation => render!(translations, path, 3).unwrap(),
            SpectralKind::Talisman => render!(translations, path).unwrap(),
            SpectralKind::Aura => render!(translations, path).unwrap(),
            SpectralKind::Wraith =>  render!(translations, path).unwrap(),
            SpectralKind::Sigil =>  render!(translations, path).unwrap(),
            SpectralKind::Ouija =>  render!(translations, path).unwrap(),
            SpectralKind::Ectoplasm { hand_size_penalty } =>  render!(translations, path, *hand_size_penalty).unwrap(),
            SpectralKind::Immolate =>  render!(translations, path, 5).unwrap(),
            SpectralKind::Ankh =>  render!(translations, path).unwrap(),
            SpectralKind::DejaVu => render!(translations, path).unwrap(),
            SpectralKind::Hex =>  render!(translations, path).unwrap(),
            SpectralKind::Trance =>  render!(translations, path).unwrap(),
            SpectralKind::Medium =>  render!(translations, path).unwrap(),
            SpectralKind::Cryptid =>  render!(translations, path, 2).unwrap(),
            SpectralKind::TheSoul =>  render!(translations, path).unwrap(),
            SpectralKind::BlackHole =>  render!(translations, path).unwrap(),
        }
    }
}