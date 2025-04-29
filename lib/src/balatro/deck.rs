use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub edition: Option<Edition>,
    pub enhancement: Option<Enhancement>,
    pub rank: Rank,
    pub suit: Suit,
    pub seal: Option<Seal>
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum Enhancement {
    #[serde(rename = "m_wild")]
    Wild,
    #[serde(rename = "m_glass")]
    Glass,
    #[serde(rename = "m_bonus")]
    Bonus,
    #[serde(rename = "m_mult")]
    Mult,
    #[serde(rename = "m_lucky")]
    Lucky,
    #[serde(rename = "m_steel")]
    Steel,
    #[serde(rename = "m_stone")]
    Stone,
    #[serde(rename = "m_gold")]
    Gold
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum Seal {
    #[serde(rename = "blue_seal")]
    Blue,
    #[serde(rename = "red_seal")]
    Red,
    #[serde(rename = "purple_seal")]
    Purple,
    #[serde(rename = "gold_seal")]
    Gold
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Edition {
    #[serde(rename = "e_base")]
    None,
    #[serde(rename = "e_foil")]
    Foil,
    #[serde(rename = "e_holo")]
    Holographic,
    #[serde(rename = "e_polychrome")]
    Polychrome
}