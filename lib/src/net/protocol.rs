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
pub struct SetupRun {
    pub back: String,
    pub stake: u32,
    pub seed: Option<String>,
}

impl Request for SetupRun {
    type Expect = Ack;
}

impl Packet for SetupRun {
    fn kind() -> &'static str {
        "setup_run"
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
