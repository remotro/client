use serde::{Deserialize, Serialize};
use crate::balatro_enum;

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
    Mercury = "c_mercury",
    Venus = "c_venus",
    Earth = "c_earth",
    Mars = "c_mars",
    Jupiter = "c_jupiter",
    Saturn = "c_saturn",
    Uranus = "c_uranus",
    Neptune = "c_neptune",
    Pluto = "c_pluto",
    PlanetX = "c_planet_x",
    Ceres = "c_ceres",
    Eris = "c_eris",
});

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
    WheelOfFortune = "c_wheel_of_fortune",
    Strength = "c_strength",
    HangedMan = "c_hanged_man",
    Death = "c_death",
    Temperance = "c_temperance",
    Devil = "c_devil",
    Tower = "c_tower",
    Star = "c_star",
    Moon = "c_moon",
    Sun = "c_sun",
    Judgement = "c_judgement",
    World = "c_world",
});

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
    Ectoplasm = "c_ectoplasm",
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
