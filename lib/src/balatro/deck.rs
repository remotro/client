use serde::{Deserialize, Serialize};
use crate::{balatro::translations::{Translatable, Translation, Translations}, balatro_enum, render};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayingCard {
    pub edition: Option<CardEdition>,
    pub enhancement: Option<Enhancement>,
    pub rank: Rank,
    pub suit: Suit,
    pub seal: Option<Seal>,
    pub extra_chips: u64
}

balatro_enum!(Suit {
    Spades = "Spades",
    Hearts = "Hearts",
    Clubs = "Clubs",
    Diamonds = "Diamonds"
});

impl Translatable for Suit {
    fn translate(&self, translations: &Translations) -> Translation {
        let path = format!["misc.suits_plural.{}", self.id()];
        return render!(translations, path).unwrap();
    }
}
balatro_enum!(Rank {
    Ace = "Ace",
    Two = "2",
    Three = "3",
    Four = "4",
    Five = "5",
    Six = "6",
    Seven = "7",
    Eight = "8",
    Nine = "9",
    Ten = "10",
    Jack = "Jack",
    Queen = "Queen",
    King = "King"
});

impl Translatable for Rank {
    fn translate(&self, translations: &Translations) -> Translation {
        let path = format!["misc.ranks.{}", self.id()];
        return render!(translations, path).unwrap();
    }
}

balatro_enum!(Enhancement {
    Wild = "m_wild",
    Glass { probability: u64 } = "m_glass",
    Bonus = "m_bonus",
    Mult = "m_mult",
    Lucky { probability: u64 } = "m_lucky",
    Steel = "m_steel",
    Stone = "m_stone",
    Gold = "m_gold"
});

impl Translatable for Enhancement {
    fn translate(&self, translations: &Translations) -> Translation {
        let path = format!["descriptions.Enhanced.{}", self.id()];
        match self {
            Self::Wild => render!(translations, path).unwrap(),
            Self::Glass { probability } => render!(translations, path, 2, *probability, 4).unwrap(),
            Self::Bonus => render!(translations, path).unwrap(),
            Self::Mult => render!(translations, path, 4).unwrap(),
            Self::Lucky { probability} => render!(translations, path, *probability, 20, 20, *probability, 20, 20).unwrap(),
            Self::Steel =>  render!(translations, path, 1.5).unwrap(),
            Self::Stone =>  render!(translations, path, 50).unwrap(),
            Self::Gold =>  render!(translations, path, 3).unwrap(),
        }
    }
}

balatro_enum!(Seal {
    Blue = "blue_seal",
    Red = "red_seal",
    Purple = "purple_seal",
    Gold = "gold_seal"
});

impl Translatable for Seal {
    fn translate(&self, translations: &Translations) -> Translation {
        let path = format!["descriptions.Other.{}", self.id()];
        render!(translations, path).unwrap()
    }
}

balatro_enum!(CardEdition {
    None = "e_base",
    Foil = "e_foil",
    Holographic = "e_holo",
    Polychrome = "e_polychrome"
});

impl Translatable for CardEdition {
    fn translate(&self, translations: &Translations) -> Translation {
        let path = format!["misc.v_dictionary.{}", self.id()];
        match self {
            CardEdition::None => render!(translations, path).unwrap(),
            CardEdition::Foil => render!(translations, path, 10).unwrap(),
            CardEdition::Holographic => render!(translations, path, 10).unwrap(),
            CardEdition::Polychrome => render!(translations, path, 1.5).unwrap()
        }
    }
}
