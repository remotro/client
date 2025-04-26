use crate::net::Connection;
use crate::balatro::blinds::SelectBlind;
use super::Screen;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub struct Menu<'a> {
    connection: &'a mut Connection,
}

impl <'a> Menu<'a> {
    pub(crate) fn new(connection: &'a mut Connection) -> Self {
        Self { connection }
    }

    pub async fn new_run(self, deck: Deck, stake: Stake, seed: Option<Seed>) -> Result<SelectBlind<'a>, super::Error> {
        let new_run = protocol::StartRun {
            back: protocol::Back(deck),
            stake: protocol::StakeNo(stake),
            seed,
        };
        let blinds = self.connection.request(new_run).await??;
        Ok(SelectBlind::new(blinds, self.connection))
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Deck {
    Red,
    Blue,
    Yellow,
    Green,
    Black,
    Magic,
    Nebula,
    Ghost,
    Abandoned,
    Checkered,
    Zodiac,
    Painted,
    Anaglyph,
    Plasma,
    Erratic
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Stake {
    White,
    Red,
    Green,
    Black,
    Blue,
    Purple,
    Orange,
    Gold
}
impl FromStr for Deck {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "red" => Ok(Deck::Red),
            "blue" => Ok(Deck::Blue),
            "yellow" => Ok(Deck::Yellow),
            "green" => Ok(Deck::Green),
            "black" => Ok(Deck::Black),
            "magic" => Ok(Deck::Magic),
            "nebula" => Ok(Deck::Nebula),
            "ghost" => Ok(Deck::Ghost),
            "abandoned" => Ok(Deck::Abandoned),
            "checkered" => Ok(Deck::Checkered),
            "zodiac" => Ok(Deck::Zodiac),
            "painted" => Ok(Deck::Painted),
            "anaglyph" => Ok(Deck::Anaglyph),
            "plasma" => Ok(Deck::Plasma),
            "erratic" => Ok(Deck::Erratic),
            _ => Err(format!("Invalid deck. Valid options are: {}", 
                ["Red", "Blue", "Yellow", "Green", "Black", "Magic", "Nebula", 
                 "Ghost", "Abandoned", "Checkered", "Zodiac", "Painted", 
                 "Anaglyph", "Plasma", "Erratic"].join(", ")))
        }
    }
}
impl FromStr for Stake {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "white" => Ok(Stake::White),
            "red" => Ok(Stake::Red),
            "green" => Ok(Stake::Green),
            "black" => Ok(Stake::Black),
            "blue" => Ok(Stake::Blue),
            "purple" => Ok(Stake::Purple),
            "orange" => Ok(Stake::Orange),
            "gold" => Ok(Stake::Gold),
            _ => Err(format!("Invalid stake. Valid options are: {}",
                ["White", "Red", "Green", "Black", "Blue", "Purple", 
                 "Orange", "Gold"].join(", ")))
        }
    }
}

#[derive(Serialize)]
pub struct Seed(String);

pub(crate) mod protocol {
    use crate::{balatro::blinds::protocol::BlindInfo, net::protocol::{Packet, Request}};
    use super::{Deck, Seed, Stake};
    use serde::{Serialize, Serializer};

    // Hide serialization impls here since they're specific to Balatro's
    // internals.

    #[derive(Serialize)]
    pub struct StartRun {
        pub back: Back,
        pub stake: StakeNo,
        pub seed: Option<Seed>,
    }

    pub struct Back(pub Deck);

    impl Serialize for Back {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(match self.0 {
                Deck::Red => "b_red",
                Deck::Blue => "b_blue",
                Deck::Yellow => "b_yellow",
                Deck::Green => "b_green",
                Deck::Black => "b_black",
                Deck::Magic => "b_magic",
                Deck::Nebula => "b_nebula",
                Deck::Ghost => "b_ghost",
                Deck::Abandoned => "b_abandoned",
                Deck::Checkered => "b_checkered",
                Deck::Zodiac => "b_zodiac",
                Deck::Painted => "b_painted",
                Deck::Anaglyph => "b_anaglyph",
                Deck::Plasma => "b_plasma",
                Deck::Erratic => "b_erratic",
            })
        }
    }

    pub struct StakeNo(pub Stake);

    impl Serialize for StakeNo {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Serialize variants as 1-based integers
            let value = match self.0 {
                Stake::White => 1,
                Stake::Red => 2,
                Stake::Green => 3,
                Stake::Black => 4,
                Stake::Blue => 5,
                Stake::Purple => 6,
                Stake::Orange => 7,
                Stake::Gold => 8,
            };
            serializer.serialize_u32(value)
        }
    }

    impl Request for StartRun {
        type Expect = Result<BlindInfo, String>;
    }
    
    impl Packet for StartRun {
        fn kind() -> String {
            "main_menu/start_run".to_string()
        }
    }

}
