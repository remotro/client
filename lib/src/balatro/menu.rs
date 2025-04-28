use crate::balatro::blinds::SelectBlind;
use crate::net::Connection;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use serde_repr::{Deserialize_repr, Serialize_repr};

pub struct Menu<'a> {
    connection: &'a mut Connection,
}

impl<'a> Menu<'a> {
    pub(crate) fn new(connection: &'a mut Connection) -> Self {
        Self { connection }
    }

    pub async fn new_run(
        self,
        deck: Deck,
        stake: Stake,
        seed: Option<Seed>,
    ) -> Result<SelectBlind<'a>, super::Error> {
        let new_run = protocol::StartRun {
            back: deck,
            stake,
            seed,
        };
        let blinds = self.connection.request(new_run).await??;
        Ok(SelectBlind::new(blinds, self.connection))
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Deck {
    #[serde(rename = "b_red")]
    Red,
    #[serde(rename = "b_blue")]
    Blue,
    #[serde(rename = "b_yellow")]
    Yellow,
    #[serde(rename = "b_green")]
    Green,
    #[serde(rename = "b_black")]
    Black,
    #[serde(rename = "b_magic")]
    Magic,
    #[serde(rename = "b_nebula")]
    Nebula,
    #[serde(rename = "b_ghost")]
    Ghost,
    #[serde(rename = "b_abandoned")]
    Abandoned,
    #[serde(rename = "b_checkered")]
    Checkered,
    #[serde(rename = "b_zodiac")]
    Zodiac,
    #[serde(rename = "b_painted")]
    Painted,
    #[serde(rename = "b_anaglyph")]
    Anaglyph,
    #[serde(rename = "b_plasma")]
    Plasma,
    #[serde(rename = "b_erratic")]
    Erratic,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Stake {
    White = 1,
    Red = 2,
    Green = 3,
    Black = 4,
    Blue = 5,
    Purple = 6,
    Orange = 7,
    Gold = 8,
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
    use super::{Deck, Seed, Stake};
    use crate::{
        balatro::blinds::protocol::BlindInfo,
        net::protocol::{Packet, Request},
    };
    use serde::Serialize;

    // Hide serialization impls here since they're specific to Balatro's
    // internals.

    #[derive(Serialize)]
    pub struct StartRun {
        pub back: Deck,
        pub stake: Stake,
        pub seed: Option<Seed>,
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
