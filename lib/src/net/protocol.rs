use serde::{Deserialize, de::DeserializeOwned, Serialize};

pub trait Packet {
    fn kind() -> &'static str;
}

pub trait Request: Serialize + Packet {
    type Expect: Response;
}

pub trait Response : DeserializeOwned + Packet {
}

#[derive(Serialize)]
pub struct NewRun {
    pub deck_id: String,
    pub stake_id: String,
    pub seed: Option<String>,
}

impl Request for NewRun {
    type Expect = Ack;
}

impl Packet for NewRun {
    fn kind() -> &'static str {
        "new_run"
    }
}

#[derive(Deserialize)]
pub struct Ack {
    pub result: Result<(), String>
}

impl Response for Ack {}

impl Packet for Ack {
    fn kind() -> &'static str {
        "ack"
    }
}
