use crate::{balatro_enum,balatro::{Screen, blinds::SelectBlind}};
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

balatro_enum! {
    Deck {
        Red = "b_red",
        Blue = "b_blue",
        Yellow = "b_yellow",
        Green = "b_green",
        Black = "b_black",
        Magic = "b_magic",
        Nebula = "b_nebula",
        Ghost = "b_ghost",
        Abandoned = "b_abandoned",
        Checkered = "b_checkered",
        Zodiac = "b_zodiac",
        Painted = "b_painted",
        Anaglyph = "b_anaglyph",
        Plasma = "b_plasma",
        Erratic = "b_erratic",
    }
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
impl Seed {
    pub fn new(s: &str) -> Self {
        Seed(s.to_string())
    }
}

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
