use serde::{Deserialize, Serialize};
use crate::balatro_enum;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayingCard {
    pub edition: Option<CardEdition>,
    pub enhancement: Option<Enhancement>,
    pub rank: Rank,
    pub suit: Suit,
    pub seal: Option<Seal>
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Rank {
    #[serde(rename = "Ace")]
    Ace,
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "3")]
    Three,
    #[serde(rename = "4")]
    Four,
    #[serde(rename = "5")]
    Five,
    #[serde(rename = "6")]
    Six,
    #[serde(rename = "7")]
    Seven,
    #[serde(rename = "8")]
    Eight,
    #[serde(rename = "9")]
    Nine,
    #[serde(rename = "10")]
    Ten,
    Jack,
    Queen,
    King
}

balatro_enum!(Enhancement {
    Wild = "m_wild",
    Glass = "m_glass",
    Bonus = "m_bonus",
    Mult = "m_mult",
    Lucky = "m_lucky",
    Steel = "m_steel",
    Stone = "m_stone",
    Gold = "m_gold"
});

balatro_enum!(Seal {
    Blue = "blue_seal",
    Red = "red_seal",
    Purple = "purple_seal",
    Gold = "gold_seal"
});

balatro_enum!(CardEdition {
    None = "e_base",
    Foil = "e_foil",
    Holographic = "e_holo",
    Polychrome = "e_polychrome"
});
